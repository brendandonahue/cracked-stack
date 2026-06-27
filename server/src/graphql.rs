use rocket::{post, State, serde::json::Json, http::CookieJar};
use log::info;
use crate::error::Error;
use crate::auth::AuthUser;
use std::sync::Arc;
use reqwest::Client;
use serde_json::{json, Value as SerdeValue};
use std::env;

#[post("/graphql", format = "application/json", data = "<body>")]
pub async fn graphql_proxy(
    _user: AuthUser,  // Ensures auth
    http_client: &State<Arc<Client>>,
    cookies: &CookieJar<'_>,
    body: Json<SerdeValue>,  // GraphQL query body
) -> Result<Json<SerdeValue>, Error> {
    let host = env::var("SURREALDB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("SURREALDB_PORT").unwrap_or_else(|_| "8001".to_string());
    let port: u16 = port_str.parse().map_err(|_| Error::Internal("Invalid port".to_string()))?;  // Parse port safely

    let ns = env::var("SURREALDB_NS").map_err(|_| Error::Internal("SURREALDB_NS not set".to_string()))?;
    let db_name = env::var("SURREALDB_DB").map_err(|_| Error::Internal("SURREALDB_DB not set".to_string()))?;

    let client = http_client.inner().clone();  // Clone the client for use

    // Extract user token from cookies (already validated by AuthUser guard)
    let token = cookies.get("auth_token")
        .map(|cookie| cookie.value().trim().to_string())
        .unwrap_or_default();

    if token.is_empty() {
        return Err(Error::Internal("No auth token found".to_string()));
    }

    // Forward the GraphQL query
    let response: SerdeValue = client.post(format!("http://{}:{}/graphql", host, port))  // Assuming /sql for GraphQL, but use /graphql if direct
        .header("Authorization", format!("Bearer {}", token))
        .header("Surreal-NS", ns)
        .header("Surreal-DB", db_name)
        .header("Accept", "application/json")
        .json(&body.0)  // Forward the body
        .send()
        .await
        .map_err(|e| Error::Internal(format!("GraphQL request failed: {}", e)))?
        .json()
        .await
        .map_err(|e| Error::Internal(format!("Response parse failed: {}", e)))?;

    info!("GraphQL proxy response: {:?}", response);
    Ok(Json(response))
}