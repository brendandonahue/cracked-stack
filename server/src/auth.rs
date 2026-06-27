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
    pub client: Surreal<Client>,
    /// Namespace/database-selected client WITHOUT root auth — used for record-access
    /// signup/login so the SIGNUP/SIGNIN clauses run in the correct session context.
    pub record_client: Surreal<Client>,
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

    /// Returns JWT token on successful login
    pub async fn login(&self, email: String, password: String) -> Result<String, Error> {
        info!("Login attempt for email: {}", email);

        let ns = env::var("SURREALDB_NS").expect("SURREALDB_NS env var is required");
        let db_name = env::var("SURREALDB_DB").expect("SURREALDB_DB env var is required");

        let token = self.record_client
            .signin(Record {
                namespace: ns,
                database: db_name,
                access: "account".to_string(),
                params: LoginParams {
                    email: email.clone(),
                    pass: password,
                },
            })
            .await
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

        let token = self.record_client
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
            .await
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
        let client = &db_inner.client;

        // Extract token
        let token = if let Some(cookie) = req.cookies().get("auth_token") {
            cookie.value().trim().to_string()
        } else if let Some(header) = req.headers().get_one("Authorization") {
            header.strip_prefix("Bearer ")
                .map(|t| t.trim().to_string())
                .unwrap_or_default()
        } else {
            return Outcome::Error((Status::Unauthorized, Status::Unauthorized));
        };

        if token.is_empty() {
            return Outcome::Error((Status::Unauthorized, Status::Unauthorized));
        }

        // Acquire the auth_guard lock so no concurrent operation changes client auth state
        // while we authenticate as a user, query, and then restore root auth.
        let _guard = db_inner.auth_guard.lock().await;

        // Authenticate the shared client as this user token
        if let Err(e) = client.authenticate(token).await {
            info!("Token auth failed: {:?}", e);
            // Restore root auth before releasing lock
            let _ = Self::restore_root(db_inner).await;
            return Outcome::Error((Status::Unauthorized, Status::Unauthorized));
        }

        // Get current user from $auth (runs in user-auth context)
        let user_result: Result<Vec<AuthUser>, _> = client
            .query("SELECT id, name, email, role, company_id FROM $auth LIMIT 1")
            .await
            .and_then(|mut response| response.take(0));

        // Always restore root auth before releasing the lock, regardless of query outcome
        let _ = Self::restore_root(db_inner).await;

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

impl AuthUser {
    /// Re-authenticate the shared client as root and reselect ns/db.
    /// Called after user-level authenticate() to restore privileged access.
    async fn restore_root(db: &Db) -> Result<(), ()> {
        let user = env::var("SURREALDB_USER").unwrap_or_default();
        let pass = env::var("SURREALDB_PASS").unwrap_or_default();
        let _ = db.client.signin(Root { username: user, password: pass }).await;
        let _ = db.client.use_ns(&db.ns).use_db(&db.db_name).await;
        Ok(())
    }
}
