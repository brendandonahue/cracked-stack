use rocket::{get, post, routes, State, serde::json::Json};
use rocket_cors::{
    AllowedHeaders, AllowedOrigins, CorsOptions
};
use rocket::http::{Status, Method};
use rocket::data::{Limits, ToByteUnit};
use rocket::shield::Shield;
use rocket::figment::{Figment, providers::{Serialized, Env}};
use log::{error, info};
use serde::{Serialize, Deserialize};
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use reqwest::Client;
mod graphql;
mod notifications;
mod error;
use error::Error;
mod auth;
use auth::{
    Db,
    AuthUser,
};
mod uploads;
use uploads::{upload, get_file, delete_file};
use graphql::graphql_proxy;
use notifications::send_password_reset_email;

use rocket_governor::{Method as M, Quota, RocketGovernable, RocketGovernor};
use rocket::http::{Cookie, CookieJar, SameSite};
use time::Duration;
use surrealdb_types::ToSql;
use surrealdb::types::RecordId;
use surrealdb_types::SurrealValue;
use chrono::{DateTime, Utc};
use rand::Rng;
use base64::{engine::general_purpose, Engine as _};

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: M, _route_name: &str) -> Quota {
        Quota::per_second(Self::nonzero(1u32))
    }
}

#[derive(Deserialize)]
struct LoginInput {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct SignupInput {
    email: String,
    password: String,
    name: Option<String>,
    role: String,
}

#[derive(Deserialize)]
struct RequestPasswordResetInput {
    email: String,
}

#[derive(Deserialize)]
struct ResetPasswordInput {
    token: String,
    new_password: String,
}

#[derive(Deserialize, SurrealValue)]
struct UserId {
    id: RecordId,
}

#[derive(Deserialize, SurrealValue)]
struct PasswordResetToken {
    id: RecordId,
    token: String,
    email: String,
    expires_at: DateTime<Utc>,
    used: bool,
}

#[derive(Serialize, SurrealValue)]
struct PasswordResetTokenCreate {
    token: String,
    email: String,
    expires_at: DateTime<Utc>,
    used: bool,
}

#[derive(Deserialize, SurrealValue)]
struct User {
    id: RecordId,
    email: String,
    pass: String,
    role: String,
}

#[post("/login", data = "<input>")]
async fn login(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    db: &State<Db>,
    input: Json<LoginInput>,
    cookies: &CookieJar<'_>,
) -> Result<Json<serde_json::Value>, Error> {
    info!("Login request received: email={}", input.email);
    let token = db.login(input.email.clone(), input.password.clone()).await?;

    cookies.add(
        Cookie::build(("auth_token", token.clone()))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .path("/")
            .max_age(Duration::hours(24))
            .finish(),
    );

    info!("auth_token cookie set successfully");
    Ok(Json(json!({"success": true})))
}

#[post("/signup", data = "<input>")]
async fn signup(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    db: &State<Db>,
    input: Json<SignupInput>,
    cookies: &CookieJar<'_>,
) -> Result<Json<serde_json::Value>, Error> {
    info!("Signup request received: email={}, name={:?}, role={}", input.email, input.name, input.role);

    // Only allow "user" role on public signup; "admin" is set manually in the DB.
    let role = if input.role == "admin" { "user".to_string() } else { input.role.clone() };

    let token = db.signup(
        input.email.clone(),
        input.password.clone(),
        input.name.clone(),
        role,
    ).await?;

    cookies.add(
        Cookie::build(("auth_token", token.clone()))
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .path("/")
            .max_age(Duration::hours(24))
            .finish(),
    );

    Ok(Json(json!({"success": true})))
}

#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Json<serde_json::Value> {
    cookies.remove(Cookie::build("auth_token").path("/").finish());
    Json(json!({"success": true}))
}

#[get("/profile")]
async fn profile(user: AuthUser) -> Result<Json<Value>, Error> {
    Ok(Json(json!({
        "id": user.id.to_sql(),
        "email": user.email,
        "name": user.name,
        "role": user.role,
    })))
}

#[get("/validate")]
fn validate(_limitguard: RocketGovernor<'_, RateLimitGuard>, _user: AuthUser) -> Json<bool> {
    Json(true)
}

#[get("/health")]
fn health(_limitguard: RocketGovernor<'_, RateLimitGuard>) -> Status {
    Status::Ok
}

#[post("/request-password-reset", data = "<input>")]
async fn request_password_reset(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    db: &State<Db>,
    input: Json<RequestPasswordResetInput>,
) -> Result<Json<serde_json::Value>, Error> {
    info!("Password reset request received: email={}", input.email);

    let db_client = &db.client;
    let user_exists: Vec<UserId> = db_client
        .query("SELECT id FROM user WHERE email = $email LIMIT 1")
        .bind(("email", input.email.clone()))
        .await
        .map_err(|e| {
            error!("Database query failed: {:?}", e);
            Error::Internal("Database error".to_string())
        })?
        .take(0)
        .map_err(|e| {
            error!("Failed to take query result: {:?}", e);
            Error::Internal("Database error".to_string())
        })?;

    if user_exists.is_empty() {
        // Return success even if user doesn't exist (prevent email enumeration)
        info!("Password reset requested for non-existent email: {}", input.email);
        return Ok(Json(json!({"success": true})));
    }

    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let token = general_purpose::URL_SAFE_NO_PAD.encode(&bytes);

    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

    let reset_token_data = PasswordResetTokenCreate {
        token: token.clone(),
        email: input.email.clone(),
        expires_at,
        used: false,
    };

    let _created: PasswordResetTokenCreate = db_client
        .create("password_reset_tokens")
        .content(reset_token_data)
        .await
        .map_err(|e| {
            error!("Failed to create password reset token: {:?}", e);
            Error::Internal("Failed to create reset token".to_string())
        })?
        .ok_or_else(|| Error::BadRequest("Couldn't create password reset token".to_string()))?;

    send_password_reset_email(&input.email, &token).await?;

    info!("Password reset email sent to: {}", input.email);
    Ok(Json(json!({"success": true})))
}

#[post("/reset-password", data = "<input>")]
async fn reset_password(
    _limitguard: RocketGovernor<'_, RateLimitGuard>,
    db: &State<Db>,
    input: Json<ResetPasswordInput>,
) -> Result<Json<serde_json::Value>, Error> {
    info!("Password reset attempt with token");

    let db_client = &db.client;

    let tokens: Vec<PasswordResetToken> = db_client
        .query(r#"
            SELECT * FROM password_reset_tokens
            WHERE token = $reset_token AND used = false
            LIMIT 1
        "#)
        .bind(("reset_token", input.token.clone()))
        .await
        .map_err(|e| {
            error!("Database query failed: {:?}", e);
            Error::Internal("Database error".to_string())
        })?
        .take(0)
        .map_err(|e| {
            error!("Failed to take query result: {:?}", e);
            Error::Internal("Database error".to_string())
        })?;

    if tokens.is_empty() {
        return Err(Error::PasswordResetTokenNotFound);
    }

    let token_record = &tokens[0];
    let email = token_record.email.clone();
    let expires_at = token_record.expires_at;

    if chrono::Utc::now() > expires_at {
        return Err(Error::PasswordResetTokenExpired);
    }

    let _: Vec<User> = db_client
        .query(r#"
            UPDATE user SET pass = crypto::argon2::generate($password) WHERE email = $email
        "#)
        .bind(("password", input.new_password.clone()))
        .bind(("email", email.clone()))
        .await
        .map_err(|e| {
            error!("Failed to update password: {:?}", e);
            Error::Internal("Failed to update password".to_string())
        })?
        .take(0)
        .map_err(|e| {
            error!("Failed to take update result: {:?}", e);
            Error::Internal("Failed to update password".to_string())
        })?;

    let _: Vec<PasswordResetToken> = db_client
        .query(r#"
            UPDATE password_reset_tokens SET used = true WHERE token = $reset_token
        "#)
        .bind(("reset_token", input.token.clone()))
        .await
        .map_err(|e| {
            error!("Failed to mark token as used: {:?}", e);
            Error::Internal("Failed to invalidate token".to_string())
        })?
        .take(0)
        .map_err(|e| {
            error!("Failed to take token update result: {:?}", e);
            Error::Internal("Failed to invalidate token".to_string())
        })?;

    info!("Password reset successful for email: {}", email);
    Ok(Json(json!({"success": true})))
}

#[rocket::main]
async fn main() {
    dotenvy::dotenv().ok();
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:1420",
        "https://crackedstack.com",
    ]);
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("error while building CORS");

    env_logger::init_from_env(
        env_logger::Env::default()
            .default_filter_or("info")
    );

    let db_state = match Db::connect().await {
        Ok(db) => db,
        Err(e) => {
            log::error!("Failed to connect to SurrealDB: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = db_state.ensure_connected().await {
        log::error!("SurrealDB authentication failed: {}", e);
        std::process::exit(1);
    }

    info!("✅ SurrealDB initialized successfully");

    let limits = Limits::new()
        .limit("data", 2_u64.mebibytes())
        .limit("file", 10_u64.mebibytes());


    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(rocket::Config {
            address: std::net::IpAddr::from([0, 0, 0, 0]),
            port: 8000,
            ..Default::default()
        }))
        .merge(Env::prefixed("ROCKET_").split("__"))
        .merge(("limits", limits));

    let http_client = Client::builder()
        .user_agent("cracked-stack/1.0.0 (+https://github.com/brendandonahue/cracked-stack)")
        .build()
        .expect("Failed to build reqwest client");

    rocket::custom(figment)
        .attach(cors)
        .attach(Shield::default())
        .manage(Arc::new(http_client))
        .manage(db_state)
        .mount("/", routes![
            login,
            signup,
            logout,
            profile,
            validate,
            health,
            request_password_reset,
            reset_password,
            graphql_proxy,
            upload,
            get_file,
            delete_file,
        ])

        .launch()
        .await
        .unwrap();
}
