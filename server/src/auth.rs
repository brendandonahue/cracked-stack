use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::{Record, Root};
use surrealdb::Surreal;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use log::{info, error};
use std::env;
use crate::error::Error;
use surrealdb::types::{RecordId};
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct Db {
    /// Root-authenticated client — used for all privileged server-side queries.
    /// NEVER re-authenticate this connection after startup: route handlers run
    /// queries on it outside `auth_guard`, so swapping its identity races.
    pub client: Surreal<Client>,
    /// Namespace/database-selected client WITHOUT root auth — used for record-access
    /// signup/login so the SIGNUP/SIGNIN clauses run in the correct session context,
    /// and for validating user JWTs in the `AuthUser` guard.
    pub record_client: Surreal<Client>,
    /// Serializes every use of `record_client` (login, signup, token validation)
    /// because each one mutates that connection's session identity.
    pub auth_guard: Arc<Mutex<()>>,
    ns: String,
    db_name: String,
}

#[derive(Serialize, Deserialize, SurrealValue)]
struct LoginParams {
    email: String,
    pass: String,
}

#[derive(Serialize, Deserialize, SurrealValue)]
struct SignupParams {
    email: String,
    pass: String,
    name: String,
    role: String,
}

impl Db {
    pub async fn connect() -> Result<Self, Error> {
        let host = env::var("SURREALDB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("SURREALDB_PORT").unwrap_or_else(|_| "8001".to_string());
        let url = format!("{}:{}", host, port);

        info!("Attempting SurrealDB connection to {}", url);

        // Root client — will be root-authenticated in ensure_connected()
        let client: Surreal<Client> = Surreal::new::<Ws>(&url)
            .await
            .map_err(|e| {
                error!("Failed to create SurrealDB root client: {}", e);
                Error::Internal(format!("SurrealDB connection failed: {}", e))
            })?;

        // Record client — separate connection, only ns/db selected, no root auth.
        // Used for signup/login record-access operations.
        let record_client: Surreal<Client> = Surreal::new::<Ws>(&url)
            .await
            .map_err(|e| {
                error!("Failed to create SurrealDB record client: {}", e);
                Error::Internal(format!("SurrealDB record connection failed: {}", e))
            })?;

        let ns = env::var("SURREALDB_NS").expect("SURREALDB_NS required");
        let db_name = env::var("SURREALDB_DB").expect("SURREALDB_DB required");

        Ok(Self {
            client,
            record_client,
            auth_guard: Arc::new(Mutex::new(())),
            ns,
            db_name,
        })
    }

    /// Authenticates the root client and selects the namespace/database.
    /// Also selects the namespace/database on the record_client (without root auth).
    /// Used at startup only.
    pub async fn ensure_connected(&self) -> Result<(), Error> {
        let _guard = self.auth_guard.lock().await;
        self.reauth_root_inner().await?;

        // Record client: only ns/db — no root auth, so SIGNUP/SIGNIN clauses
        // execute in the correct unauthenticated database-level session context.
        self.record_client.use_ns(&self.ns)
            .use_db(&self.db_name)
            .await
            .map_err(|e| {
                error!("Failed to select namespace/database (record client): {:?}", e);
                Error::from(e)
            })?;

        info!("✅ SurrealDB root auth + ns/db selection successful");
        Ok(())
    }

    /// Re-authenticates the root client as root and reselects ns/db.
    /// Caller MUST hold `auth_guard` before calling this.
    /// Used to restore root auth after a user-level authenticate() call.
    pub async fn reauth_root_inner(&self) -> Result<(), Error> {
        let user = env::var("SURREALDB_USER").expect("SURREALDB_USER required");
        let pass = env::var("SURREALDB_PASS").expect("SURREALDB_PASS required");

        self.client.signin(Root {
            username: user,
            password: pass,
        })
        .await
        .map_err(|e| {
            error!("Root signin failed: {:?}", e);
            Error::from(e)
        })?;

        self.client.use_ns(&self.ns)
            .use_db(&self.db_name)
            .await
            .map_err(|e| {
                error!("Failed to select namespace/database (root client): {:?}", e);
                Error::from(e)
            })?;

        Ok(())
    }

    /// Resets `record_client` to its unauthenticated, ns/db-selected baseline.
    /// Caller MUST hold `auth_guard`.
    async fn reset_record_client(&self) {
        let _ = self.record_client.invalidate().await;
        let _ = self.record_client.use_ns(&self.ns).use_db(&self.db_name).await;
    }

    /// Returns JWT token on successful login
    pub async fn login(&self, email: String, password: String) -> Result<String, Error> {
        info!("Login attempt for email: {}", email);

        let ns = env::var("SURREALDB_NS").expect("SURREALDB_NS env var is required");
        let db_name = env::var("SURREALDB_DB").expect("SURREALDB_DB env var is required");

        // signin() changes `record_client`'s session identity; hold the lock so a
        // concurrent AuthUser validation can't observe (or be clobbered by) it.
        let _guard = self.auth_guard.lock().await;

        let result = self.record_client
            .signin(Record {
                namespace: ns,
                database: db_name,
                access: "account".to_string(),
                params: LoginParams {
                    email: email.clone(),
                    pass: password,
                },
            })
            .await;

        // Never leave the shared connection authenticated as this user.
        self.reset_record_client().await;

        let token = result
            .map_err(|e| {
                error!("SurrealDB signin failed for {}: {:?}", email, e);
                Error::from(e)
            })?
            .access
            .into_insecure_token();

        info!("Login successful for email: {}", email);
        Ok(token)
    }

    /// Returns JWT token on successful signup
    pub async fn signup(
        &self,
        email: String,
        password: String,
        name: Option<String>,
        role: String,
    ) -> Result<String, Error> {
        info!("Signup attempt - email: {}, name: {:?}, role: {}", email, name, role);

        let ns = env::var("SURREALDB_NS").expect("SURREALDB_NS env var is required");
        let db_name = env::var("SURREALDB_DB").expect("SURREALDB_DB env var is required");

        // See `login`: signup() also changes `record_client`'s session identity.
        let _guard = self.auth_guard.lock().await;

        let result = self.record_client
            .signup(Record {
                namespace: ns,
                database: db_name,
                access: "account".to_string(),
                params: SignupParams {
                    email: email.clone(),
                    pass: password,
                    name: name.unwrap_or_else(|| "User".to_string()),
                    role,
                },
            })
            .await;

        self.reset_record_client().await;

        let token = result
            .map_err(|e| {
                error!("SurrealDB signup failed for {}: {:?}", email, e);
                Error::from(e)
            })?
            .access
            .into_insecure_token();

        info!("Signup successful for email: {}", email);
        Ok(token)
    }
}

// ──────────────────────────────────────────────────────────────
// Request Guard: verifies the JWT and attaches the user record
// ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, SurrealValue)]
pub struct AuthUser {
    pub id: RecordId,
    pub name: Option<String>,
    pub email: String,
    pub role: String,
    pub company_id: Option<RecordId>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = Status;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Correct guard for managed state
        let db_state: &State<Db> = match req.guard::<&State<Db>>().await {
            Outcome::Success(state) => state,
            _ => {
                info!("Failed to get Db state");
                return Outcome::Error((Status::ServiceUnavailable, Status::ServiceUnavailable));
            }
        };

        let db_inner = db_state.inner();
        // IMPORTANT: token validation must NEVER touch `db_inner.client` — that
        // connection is the shared, always-root-authenticated client every route
        // handler relies on for its own queries, which run *after* this guard
        // returns and therefore outside `auth_guard`'s lock.
        //
        // The previous implementation temporarily re-authenticated
        // `db_inner.client` as the requesting user (to run `SELECT ... FROM
        // $auth`) and then signed back in as root before releasing the lock.
        // That only serialized the swap itself: request A's handler could start
        // querying `db.client` while request B's guard re-authenticated that SAME
        // shared client as user B, so A's query silently ran with B's permissions
        // or failed with a generic "Database error" (intermittent 500s under
        // concurrent requests).
        //
        // Fix: validate the token on `record_client` — a separate connection no
        // handler uses for privileged queries — and serialize every use of it
        // (login, signup, validation) under `auth_guard`.
        let client = &db_inner.record_client;

        // Extract token
        let token = match extract_token(
            req.cookies().get("auth_token").map(|c| c.value()),
            req.headers().get_one("Authorization"),
        ) {
            Some(t) => t,
            None => return Outcome::Error((Status::Unauthorized, Status::Unauthorized)),
        };

        // Hold the lock for the whole authenticate -> query -> reset sequence.
        let _guard = db_inner.auth_guard.lock().await;

        // Authenticate record_client as this user token
        if let Err(e) = client.authenticate(token).await {
            info!("Token auth failed: {:?}", e);
            db_inner.reset_record_client().await;
            return Outcome::Error((Status::Unauthorized, Status::Unauthorized));
        }

        // Get current user from $auth (runs in user-auth context)
        let user_result: Result<Vec<AuthUser>, _> = client
            .query("SELECT id, name, email, role, company_id FROM $auth LIMIT 1")
            .await
            .and_then(|mut response| response.take(0));

        // Always reset record_client to its unauthenticated baseline before
        // releasing the lock, regardless of query outcome.
        db_inner.reset_record_client().await;

        match user_result {
            Ok(mut users) if !users.is_empty() => {
                let user = users.remove(0);
                info!("Authenticated user: {}", user.email);
                Outcome::Success(user)
            }
            _ => {
                info!("No $auth record found");
                Outcome::Error((Status::Unauthorized, Status::Unauthorized))
            }
        }
    }
}

/// Extracts the JWT from the `auth_token` cookie (preferred) or an
/// `Authorization: Bearer <token>` header. Returns `None` if absent or empty.
fn extract_token(cookie: Option<&str>, auth_header: Option<&str>) -> Option<String> {
    let token = if let Some(c) = cookie {
        c.trim().to_string()
    } else if let Some(h) = auth_header {
        h.strip_prefix("Bearer ").map(|t| t.trim().to_string()).unwrap_or_default()
    } else {
        return None;
    };
    if token.is_empty() { None } else { Some(token) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_token_prefers_cookie() {
        assert_eq!(extract_token(Some(" abc "), Some("Bearer xyz")), Some("abc".to_string()));
    }

    #[test]
    fn extract_token_falls_back_to_bearer_header() {
        assert_eq!(extract_token(None, Some("Bearer xyz ")), Some("xyz".to_string()));
    }

    #[test]
    fn extract_token_rejects_missing_empty_or_malformed() {
        assert_eq!(extract_token(None, None), None);
        assert_eq!(extract_token(Some("  "), None), None);
        assert_eq!(extract_token(None, Some("Bearer ")), None);
        assert_eq!(extract_token(None, Some("Basic abc")), None);
    }
}
