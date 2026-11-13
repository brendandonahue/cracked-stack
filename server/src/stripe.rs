// server/src/stripe.rs
//
// Small custom Stripe integration (no SDK — plain REST calls via reqwest).
//
// - GET  /stripe/plans     — public. Server-side catalogue of demo plans. Prices live
//                            here (never in the client) so amounts can't be tampered with.
// - POST /stripe/checkout  — authenticated. Creates a hosted Stripe Checkout Session for
//                            a plan and returns `{ url }` for the browser to redirect to.
// - POST /stripe/webhook   — called by Stripe. Verifies the `Stripe-Signature` header
//                            (HMAC-SHA256 over `<timestamp>.<raw body>`) and records a
//                            `payment` row once Stripe confirms the session is paid.
//
// Payments are ONLY written by the webhook handler, on the root `db.client`. The
// `payment` table has no create/update/delete permissions for users (see schema.surql),
// so a payment can't be forged through the GraphQL proxy.
//
// Env: STRIPE_SECRET_KEY, STRIPE_WEBHOOK_SECRET, APP_BASE_URL (falls back to
// FRONTEND_URL, then http://localhost:1420). Secrets are never logged.

use rocket::data::{Data, ToByteUnit};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::{get, post, State};
use rocket_governor::RocketGovernor;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::Deserialize;
use serde_json::{json, Value};
use surrealdb::types::{RecordId, RecordIdKey};
use surrealdb_types::ToSql;
use reqwest::Client;
use log::{error, info, warn};
use std::env;
use std::sync::Arc;

use crate::auth::{AuthUser, Db};
use crate::error::Error;
use crate::RateLimitGuard;

const STRIPE_CHECKOUT_URL: &str = "https://api.stripe.com/v1/checkout/sessions";

/// Stripe recommends rejecting events whose timestamp is more than 5 minutes off.
const SIGNATURE_TOLERANCE_SECS: i64 = 300;

/// Webhook bodies are small JSON events; 1 MiB is far more than needed.
const MAX_WEBHOOK_MIB: u64 = 1;

/// A purchasable demo plan. Amounts are in the smallest currency unit (cents).
pub struct Plan {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub amount: i64,
    pub currency: &'static str,
}

/// Replace with your own products. Uses inline Stripe `price_data`, so no
/// Dashboard products/prices need to be created to try the demo.
pub static PLANS: [Plan; 3] = [
    Plan {
        id: "supporter",
        name: "Supporter",
        description: "A small one-time thank-you payment.",
        amount: 500,
        currency: "usd",
    },
    Plan {
        id: "pro",
        name: "Pro",
        description: "A mid-sized one-time demo payment.",
        amount: 1900,
        currency: "usd",
    },
    Plan {
        id: "team",
        name: "Team",
        description: "A larger one-time demo payment.",
        amount: 4900,
        currency: "usd",
    },
];

fn find_plan(id: &str) -> Option<&'static Plan> {
    PLANS.iter().find(|p| p.id == id)
}

fn plan_json(plan: &Plan) -> Value {
    json!({
        "id": plan.id,
        "name": plan.name,
        "description": plan.description,
        "amount": plan.amount,
        "currency": plan.currency,
    })
}

fn non_empty_env(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn secret_key() -> Result<String, Error> {
    non_empty_env("STRIPE_SECRET_KEY").ok_or(Error::StripeNotConfigured)
}

fn webhook_secret() -> Result<String, Error> {
    non_empty_env("STRIPE_WEBHOOK_SECRET").ok_or(Error::StripeNotConfigured)
}

/// Base URL Stripe redirects back to after checkout (no trailing slash).
fn app_base_url() -> String {
    non_empty_env("APP_BASE_URL")
        .or_else(|| non_empty_env("FRONTEND_URL"))
        .unwrap_or_else(|| "http://localhost:1420".to_string())
        .trim_end_matches('/')
        .to_string()
}

/// The part of a record id after the table (e.g. `user:abc` -> `abc`).
fn record_key(id: &RecordId) -> String {
    match &id.key {
        RecordIdKey::String(s) => s.clone(),
        RecordIdKey::Number(n) => n.to_string(),
        other => other.to_sql(),
    }
}

/// Form fields for `POST /v1/checkout/sessions` (Stripe uses form encoding).
fn checkout_form(plan: &Plan, base_url: &str, user_key: &str, email: &str) -> Vec<(String, String)> {
    let field = |k: &str, v: &str| (k.to_string(), v.to_string());
    vec![
        field("mode", "payment"),
        field("success_url", &format!("{}/payments?checkout=success", base_url)),
        field("cancel_url", &format!("{}/payments?checkout=cancelled", base_url)),
        field("client_reference_id", user_key),
        field("customer_email", email),
        field("metadata[plan_id]", plan.id),
        field("line_items[0][quantity]", "1"),
        field("line_items[0][price_data][currency]", plan.currency),
        field("line_items[0][price_data][unit_amount]", &plan.amount.to_string()),
        field("line_items[0][price_data][product_data][name]", plan.name),
        field("line_items[0][price_data][product_data][description]", plan.description),
    ]
}

fn stripe_error_message(body: &Value) -> String {
    body.get("error")
        .and_then(|e| e.get("message"))
        .and_then(Value::as_str)
        .unwrap_or("unknown Stripe error")
        .to_string()
}

#[get("/stripe/plans")]
pub fn plans() -> Json<Value> {
    Json(json!({
        "configured": non_empty_env("STRIPE_SECRET_KEY").is_some(),
        "plans": PLANS.iter().map(plan_json).collect::<Vec<_>>(),
    }))
}

#[derive(Deserialize)]
pub struct CheckoutInput {
    plan_id: String,
}

#[post("/stripe/checkout", data = "<input>")]
pub async fn create_checkout(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    user: AuthUser,
    http_client: &State<Arc<Client>>,
    input: Json<CheckoutInput>,
) -> Result<Json<Value>, Error> {
    let secret = secret_key()?;
    let plan = find_plan(&input.plan_id)
        .ok_or_else(|| Error::BadRequest("Unknown plan".to_string()))?;

    let form = checkout_form(plan, &app_base_url(), &record_key(&user.id), &user.email);

    let response = http_client
        .inner()
        .post(STRIPE_CHECKOUT_URL)
        .bearer_auth(&secret)
        .form(&form)
        .send()
        .await
        .map_err(|e| {
            error!("Stripe checkout request failed: {}", e);
            Error::BadGateway
        })?;

    let status = response.status();
    let body: Value = response.json().await.map_err(|e| {
        error!("Stripe checkout response parse failed: {}", e);
        Error::BadGateway
    })?;

    if !status.is_success() {
        error!("Stripe checkout session creation failed ({}): {}", status, stripe_error_message(&body));
        return Err(Error::BadGateway);
    }

    let url = body.get("url").and_then(Value::as_str).ok_or_else(|| {
        error!("Stripe checkout response had no url");
        Error::BadGateway
    })?;

    info!("Stripe checkout session created: plan={} user={}", plan.id, user.email);
    Ok(Json(json!({ "url": url })))
}


// ──────────────────────────────────────────────────────────────
// Webhook
// ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum SignatureError {
    Malformed,
    Mismatch,
    Expired,
}

/// Verifies a `Stripe-Signature` header (`t=<ts>,v1=<hex>[,v1=<hex>...]`).
/// Signed payload is `<ts>.<raw body>`, HMAC-SHA256 keyed with the endpoint secret.
fn verify_signature(
    payload: &[u8],
    header: &str,
    secret: &str,
    now: i64,
    tolerance: i64,
) -> Result<(), SignatureError> {
    let mut timestamp: Option<i64> = None;
    let mut signatures: Vec<Vec<u8>> = Vec::new();

    for part in header.split(',') {
        let Some((key, value)) = part.trim().split_once('=') else { continue };
        match key {
            "t" => timestamp = value.parse().ok(),
            "v1" => {
                if let Ok(bytes) = hex::decode(value) {
                    signatures.push(bytes);
                }
            }
            _ => {}
        }
    }

    let timestamp = timestamp.ok_or(SignatureError::Malformed)?;
    if signatures.is_empty() {
        return Err(SignatureError::Malformed);
    }

    let mut signed = timestamp.to_string().into_bytes();
    signed.push(b'.');
    signed.extend_from_slice(payload);

    let matches = signatures.iter().any(|sig| {
        match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
            // `verify_slice` compares in constant time.
            Ok(mut mac) => {
                mac.update(&signed);
                mac.verify_slice(sig).is_ok()
            }
            Err(_) => false,
        }
    });
    if !matches {
        return Err(SignatureError::Mismatch);
    }

    if (now - timestamp).abs() > tolerance {
        return Err(SignatureError::Expired);
    }

    Ok(())
}

/// Request guard that extracts the raw `Stripe-Signature` header.
pub struct StripeSignature(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StripeSignature {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Stripe-Signature") {
            Some(value) if !value.trim().is_empty() => {
                Outcome::Success(StripeSignature(value.to_string()))
            }
            _ => Outcome::Error((Status::BadRequest, Status::BadRequest)),
        }
    }
}


/// The fields we persist from a paid Checkout Session.
#[derive(Debug, PartialEq)]
struct PaidSession {
    session_id: String,
    owner_key: String,
    plan_id: String,
    plan_name: String,
    amount: i64,
    currency: String,
}

/// Returns `Some` only for events that mean "money was collected" for a session we
/// created (it carries our `client_reference_id`). Everything else is ignored.
fn parse_paid_session(event: &Value) -> Option<PaidSession> {
    let kind = event.get("type")?.as_str()?;
    if kind != "checkout.session.completed" && kind != "checkout.session.async_payment_succeeded" {
        return None;
    }

    let object = event.get("data")?.get("object")?;
    if object.get("payment_status")?.as_str()? != "paid" {
        return None;
    }

    let session_id = object.get("id")?.as_str()?.to_string();
    let owner_key = object.get("client_reference_id")?.as_str()?.to_string();
    if session_id.is_empty() || owner_key.is_empty() {
        return None;
    }

    let plan_id = object
        .get("metadata")
        .and_then(|m| m.get("plan_id"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let plan_name = find_plan(&plan_id)
        .map(|p| p.name.to_string())
        .unwrap_or_else(|| "Unknown plan".to_string());

    Some(PaidSession {
        session_id,
        owner_key,
        plan_id,
        plan_name,
        amount: object.get("amount_total")?.as_i64()?,
        currency: object.get("currency")?.as_str()?.to_lowercase(),
    })
}

/// Idempotent: the record id is the Checkout Session id, so Stripe re-delivering the
/// same event (or `completed` followed by `async_payment_succeeded`) updates one row.
async fn record_payment<C: surrealdb::Connection>(client: &surrealdb::Surreal<C>, paid: &PaidSession) -> Result<(), Error> {
    client
        .query("UPSERT $rid SET session_id = $session_id, owner = $owner, plan_id = $plan_id, plan_name = $plan_name, amount = $amount, currency = $currency, status = 'paid'")
        .bind(("rid", RecordId::new("payment", paid.session_id.clone())))
        .bind(("session_id", paid.session_id.clone()))
        .bind(("owner", RecordId::new("user", paid.owner_key.clone())))
        .bind(("plan_id", paid.plan_id.clone()))
        .bind(("plan_name", paid.plan_name.clone()))
        .bind(("amount", paid.amount))
        .bind(("currency", paid.currency.clone()))
        .await
        .map_err(|e| {
            error!("Failed to record payment {}: {:?}", paid.session_id, e);
            Error::Internal("Failed to record payment".to_string())
        })?
        .check()
        .map_err(|e| {
            error!("Payment upsert rejected for {}: {:?}", paid.session_id, e);
            Error::Internal("Failed to record payment".to_string())
        })?;
    Ok(())
}

#[post("/stripe/webhook", data = "<payload>")]
pub async fn webhook(
    signature: StripeSignature,
    db: &State<Db>,
    payload: Data<'_>,
) -> Result<Json<Value>, Error> {
    let secret = webhook_secret()?;

    let body = payload
        .open(MAX_WEBHOOK_MIB.mebibytes())
        .into_bytes()
        .await
        .map_err(|e| {
            error!("Failed to read Stripe webhook body: {:?}", e);
            Error::BadRequest("Could not read request body".to_string())
        })?;
    if !body.is_complete() {
        return Err(Error::PayloadTooLarge);
    }
    let body = body.into_inner();

    verify_signature(
        &body,
        &signature.0,
        &secret,
        chrono::Utc::now().timestamp(),
        SIGNATURE_TOLERANCE_SECS,
    )
    .map_err(|e| {
        warn!("Rejected Stripe webhook: {:?}", e);
        Error::InvalidStripeSignature
    })?;

    let event: Value = serde_json::from_slice(&body)
        .map_err(|_| Error::BadRequest("Invalid JSON body".to_string()))?;

    match parse_paid_session(&event) {
        Some(paid) => {
            record_payment(&db.client, &paid).await?;
            info!("Stripe payment recorded: session={} plan={} amount={}", paid.session_id, paid.plan_id, paid.amount);
        }
        None => {
            info!(
                "Stripe webhook ignored: type={}",
                event.get("type").and_then(Value::as_str).unwrap_or("unknown")
            );
        }
    }

    Ok(Json(json!({ "received": true })))
}


#[cfg(test)]
mod tests {
    use super::*;
    use surrealdb_types::SurrealValue;

    const SECRET: &str = "whsec_test_secret";

    fn sign(payload: &str, timestamp: i64, secret: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(format!("{}.{}", timestamp, payload).as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    fn header(payload: &str, timestamp: i64, secret: &str) -> String {
        format!("t={},v1={}", timestamp, sign(payload, timestamp, secret))
    }

    fn paid_event(kind: &str, payment_status: &str) -> Value {
        json!({
            "id": "evt_1",
            "type": kind,
            "data": { "object": {
                "id": "cs_test_abc",
                "payment_status": payment_status,
                "client_reference_id": "user123",
                "amount_total": 1900,
                "currency": "USD",
                "metadata": { "plan_id": "pro" }
            }}
        })
    }

    #[test]
    fn plans_have_unique_ids_and_positive_amounts() {
        let mut ids: Vec<&str> = PLANS.iter().map(|p| p.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), PLANS.len());
        assert!(PLANS.iter().all(|p| p.amount > 0 && !p.currency.is_empty()));
    }

    #[test]
    fn find_plan_matches_known_and_rejects_unknown() {
        assert_eq!(find_plan("pro").map(|p| p.amount), Some(1900));
        assert!(find_plan("free-lunch").is_none());
        assert!(find_plan("").is_none());
    }

    #[test]
    fn checkout_form_contains_server_side_price_and_user_reference() {
        let plan = find_plan("supporter").unwrap();
        let form = checkout_form(plan, "https://example.com", "abc123", "a@b.co");
        let get = |k: &str| form.iter().find(|(key, _)| key == k).map(|(_, v)| v.as_str());

        assert_eq!(get("mode"), Some("payment"));
        assert_eq!(get("client_reference_id"), Some("abc123"));
        assert_eq!(get("customer_email"), Some("a@b.co"));
        assert_eq!(get("metadata[plan_id]"), Some("supporter"));
        assert_eq!(get("line_items[0][quantity]"), Some("1"));
        assert_eq!(get("line_items[0][price_data][unit_amount]"), Some("500"));
        assert_eq!(get("line_items[0][price_data][currency]"), Some("usd"));
        assert_eq!(get("success_url"), Some("https://example.com/payments?checkout=success"));
        assert_eq!(get("cancel_url"), Some("https://example.com/payments?checkout=cancelled"));
    }

    #[test]
    fn record_key_strips_the_table_prefix() {
        assert_eq!(record_key(&RecordId::new("user", "abc")), "abc");
        assert_eq!(record_key(&RecordId::new("user", 42_i64)), "42");
    }

    #[test]
    fn stripe_error_message_reads_nested_message_or_falls_back() {
        let body = json!({ "error": { "message": "No such price" } });
        assert_eq!(stripe_error_message(&body), "No such price");
        assert_eq!(stripe_error_message(&json!({})), "unknown Stripe error");
    }

    #[test]
    fn config_helpers_read_env_and_apply_fallbacks() {
        // SAFETY: test-only env mutation; these STRIPE_/APP_BASE_URL/FRONTEND_URL vars
        // are read only by this single test within the crate's test binary.
        unsafe {
            env::remove_var("STRIPE_SECRET_KEY");
            env::remove_var("STRIPE_WEBHOOK_SECRET");
            env::remove_var("APP_BASE_URL");
            env::remove_var("FRONTEND_URL");
        }
        assert!(matches!(secret_key(), Err(Error::StripeNotConfigured)));
        assert!(matches!(webhook_secret(), Err(Error::StripeNotConfigured)));
        assert_eq!(app_base_url(), "http://localhost:1420");

        unsafe {
            env::set_var("STRIPE_SECRET_KEY", "   ");
            env::set_var("FRONTEND_URL", "https://front.example.com/");
        }
        assert!(matches!(secret_key(), Err(Error::StripeNotConfigured)));
        assert_eq!(app_base_url(), "https://front.example.com");

        unsafe {
            env::set_var("STRIPE_SECRET_KEY", "sk_test_123");
            env::set_var("STRIPE_WEBHOOK_SECRET", "whsec_123");
            env::set_var("APP_BASE_URL", "https://app.example.com");
        }
        assert_eq!(secret_key().unwrap(), "sk_test_123");
        assert_eq!(webhook_secret().unwrap(), "whsec_123");
        assert_eq!(app_base_url(), "https://app.example.com");

        unsafe {
            env::remove_var("STRIPE_SECRET_KEY");
            env::remove_var("STRIPE_WEBHOOK_SECRET");
            env::remove_var("APP_BASE_URL");
            env::remove_var("FRONTEND_URL");
        }
    }


    #[test]
    fn verify_signature_accepts_a_valid_signature() {
        let payload = r#"{"id":"evt_1"}"#;
        let h = header(payload, 1_000, SECRET);
        assert_eq!(verify_signature(payload.as_bytes(), &h, SECRET, 1_000, 300), Ok(()));
    }

    #[test]
    fn verify_signature_accepts_any_matching_v1_and_ignores_unknown_schemes() {
        let good = sign("{}", 1_000, SECRET);
        let h = format!("t=1000,v0=deadbeef,v1=00ff,v1={}", good);
        assert_eq!(verify_signature(b"{}", &h, SECRET, 1_000, 300), Ok(()));
    }

    #[test]
    fn verify_signature_rejects_tampered_payload() {
        let h = header("{\"amount\":1}", 1_000, SECRET);
        assert_eq!(
            verify_signature(b"{\"amount\":999999}", &h, SECRET, 1_000, 300),
            Err(SignatureError::Mismatch)
        );
    }

    #[test]
    fn verify_signature_rejects_wrong_secret() {
        let h = header("{}", 1_000, "whsec_other");
        assert_eq!(verify_signature(b"{}", &h, SECRET, 1_000, 300), Err(SignatureError::Mismatch));
    }

    #[test]
    fn verify_signature_rejects_stale_and_future_timestamps() {
        let h = header("{}", 1_000, SECRET);
        assert_eq!(verify_signature(b"{}", &h, SECRET, 1_301, 300), Err(SignatureError::Expired));
        assert_eq!(verify_signature(b"{}", &h, SECRET, 699, 300), Err(SignatureError::Expired));
        assert_eq!(verify_signature(b"{}", &h, SECRET, 1_300, 300), Ok(()));
    }

    #[test]
    fn verify_signature_rejects_malformed_headers() {
        for bad in ["", "garbage", "t=abc,v1=00", "t=1000", "v1=00ff", "t=1000,v1=not-hex"] {
            assert_eq!(
                verify_signature(b"{}", bad, SECRET, 1_000, 300),
                Err(SignatureError::Malformed),
                "header {:?}",
                bad
            );
        }
    }

    #[test]
    fn parse_paid_session_extracts_fields_for_a_paid_checkout() {
        let parsed = parse_paid_session(&paid_event("checkout.session.completed", "paid")).unwrap();
        assert_eq!(
            parsed,
            PaidSession {
                session_id: "cs_test_abc".to_string(),
                owner_key: "user123".to_string(),
                plan_id: "pro".to_string(),
                plan_name: "Pro".to_string(),
                amount: 1900,
                currency: "usd".to_string(),
            }
        );
    }

    #[test]
    fn parse_paid_session_handles_async_payment_success() {
        assert!(parse_paid_session(&paid_event("checkout.session.async_payment_succeeded", "paid")).is_some());
    }

    #[test]
    fn parse_paid_session_ignores_unpaid_and_unrelated_events() {
        assert!(parse_paid_session(&paid_event("checkout.session.completed", "unpaid")).is_none());
        assert!(parse_paid_session(&paid_event("checkout.session.expired", "paid")).is_none());
        assert!(parse_paid_session(&paid_event("charge.refunded", "paid")).is_none());
        assert!(parse_paid_session(&json!({})).is_none());
    }

    #[test]
    fn parse_paid_session_ignores_sessions_without_a_user_reference() {
        let mut event = paid_event("checkout.session.completed", "paid");
        event["data"]["object"]["client_reference_id"] = Value::Null;
        assert!(parse_paid_session(&event).is_none());
    }

    #[test]
    fn parse_paid_session_labels_unknown_plans() {
        let mut event = paid_event("checkout.session.completed", "paid");
        event["data"]["object"]["metadata"] = json!({});
        let parsed = parse_paid_session(&event).unwrap();
        assert_eq!(parsed.plan_id, "");
        assert_eq!(parsed.plan_name, "Unknown plan");
    }

    async fn mem_db() -> surrealdb::Surreal<surrealdb::engine::local::Db> {
        let db = surrealdb::Surreal::new::<surrealdb::engine::local::Mem>(()).await.unwrap();
        db.use_ns("t").use_db("t").await.unwrap();
        db.query(include_str!("../../schema.surql")).await.unwrap().check().unwrap();
        db.query("CREATE user:demo SET email = 'demo@example.com', name = 'Demo', pass = 'x', role = 'user', created_at = time::now()").await.unwrap().check().unwrap();
        db
    }

    #[tokio::test]
    async fn record_payment_upserts_once_per_checkout_session() {
        let db = mem_db().await;
        let paid = parse_paid_session(&paid_event("checkout.session.completed", "paid"))
            .map(|mut p| { p.owner_key = "demo".to_string(); p })
            .unwrap();

        // Stripe may deliver the same event more than once.
        record_payment(&db, &paid).await.unwrap();
        record_payment(&db, &paid).await.unwrap();

        let rows: Vec<Value> = db
            .query("SELECT session_id, plan_name, amount, currency, status, <string>owner AS owner FROM payment")
            .await
            .unwrap()
            .take(0)
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["session_id"], "cs_test_abc");
        assert_eq!(rows[0]["plan_name"], "Pro");
        assert_eq!(rows[0]["amount"], 1900);
        assert_eq!(rows[0]["currency"], "usd");
        assert_eq!(rows[0]["status"], "paid");
        assert_eq!(rows[0]["owner"], "user:demo");
    }

    #[derive(serde::Serialize, SurrealValue)]
    struct SignupParams {
        email: String,
        pass: String,
        name: String,
        role: String,
    }

    #[tokio::test]
    async fn users_can_read_only_their_own_payments_and_never_write_them() {
        let db = mem_db().await;
        db.query("CREATE user:other SET email = 'other@example.com', name = 'Other', pass = 'x', role = 'user', created_at = time::now()").await.unwrap().check().unwrap();
        for (sid, owner) in [("cs_mine", "demo"), ("cs_theirs", "other")] {
            let mut paid = parse_paid_session(&paid_event("checkout.session.completed", "paid")).unwrap();
            paid.session_id = sid.to_string();
            paid.owner_key = owner.to_string();
            record_payment(&db, &paid).await.unwrap();
        }

        // A brand-new user created through the real `account` access method.
        db.signup(surrealdb::opt::auth::Record {
            namespace: "t".to_string(),
            database: "t".to_string(),
            access: "account".to_string(),
            params: SignupParams {
                email: "new@example.com".to_string(),
                pass: "pw".to_string(),
                name: "New".to_string(),
                role: "user".to_string(),
            },
        })
        .await
        .unwrap();

        // This session is now a plain record user with no payments of its own.
        let visible: Vec<Value> = db.query("SELECT session_id FROM payment").await.unwrap().take(0).unwrap();
        assert!(visible.is_empty(), "a user must not see other users' payments, got {:?}", visible);

        for write in [
            "CREATE payment SET session_id = 'forged', owner = $auth.id, plan_id = 'team', plan_name = 'Team', amount = 4900, currency = 'usd'",
            "UPDATE payment SET amount = 0",
            "DELETE payment",
        ] {
            let _ = db.query(write).await.and_then(|r| r.check());
        }

        db.invalidate().await.unwrap();
        db.use_ns("t").use_db("t").await.unwrap();
        let rows: Vec<Value> = db.query("SELECT session_id, amount FROM payment ORDER BY session_id").await.unwrap().take(0).unwrap();
        assert_eq!(rows.len(), 2, "payments must be untouched by user writes: {:?}", rows);
        assert!(rows.iter().all(|r| r["amount"] == 1900));
    }
}