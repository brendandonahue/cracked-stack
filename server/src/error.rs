// server/src/error.rs
use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use rocket::Request;
use thiserror::Error;
use log::error as log_error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Db(#[source] surrealdb::Error),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,


    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("internal server error: {0}")]
    Internal(String),

    #[error("Bad Gateway")]
    BadGateway,

    #[error("Service Unavailable")]
    ServiceUnavailable,

    #[error("Not Found")]
    NotFound,

    #[error("Payload too large")]
    PayloadTooLarge,

    #[error("Password reset token not found")]
    PasswordResetTokenNotFound,

    #[error("Password reset token expired")]
    PasswordResetTokenExpired,

    #[error("Password reset token already used")]
    PasswordResetTokenUsed,

    #[error("Invalid password reset token")]
    InvalidPasswordResetToken,
}

impl Error {
    /// Returns the HTTP status code that corresponds to this error variant.
    /// Extracted so it can be called from unit tests without a Rocket `Request`.
    pub fn http_status(&self) -> Status {
        match self {
            Error::Unauthorized => Status::Unauthorized,
            Error::Forbidden => Status::Forbidden,
            Error::BadRequest(_) => Status::BadRequest,

            Error::Db(_) => Status::InternalServerError,
            Error::Internal(_) => Status::InternalServerError,
            Error::BadGateway => Status::BadGateway,
            Error::ServiceUnavailable => Status::ServiceUnavailable,
            Error::NotFound => Status::NotFound,
            Error::PayloadTooLarge => Status::PayloadTooLarge,
            Error::PasswordResetTokenNotFound => Status::NotFound,
            Error::PasswordResetTokenExpired => Status::BadRequest,
            Error::PasswordResetTokenUsed => Status::BadRequest,
            Error::InvalidPasswordResetToken => Status::BadRequest,
        }
    }

    /// Returns the human-readable message that will appear in the JSON body.
    pub fn message(&self) -> String {
        match self {
            Error::Unauthorized => "unauthorized".to_string(),
            Error::Forbidden => "forbidden".to_string(),
            Error::BadRequest(msg) => msg.clone(),

            Error::Db(_) => self.to_string(),
            Error::Internal(msg) => {
                log_error!("Internal server error: {}", msg);
                self.to_string()
            }
            Error::BadGateway => "Bad Gateway".to_string(),
            Error::ServiceUnavailable => "Service Unavailable".to_string(),
            Error::NotFound => "Not Found".to_string(),
            Error::PayloadTooLarge => "Payload Too Large".to_string(),
            Error::PasswordResetTokenNotFound => "Password reset token not found".to_string(),
            Error::PasswordResetTokenExpired => "Password reset token expired".to_string(),
            Error::PasswordResetTokenUsed => "Password reset token already used".to_string(),
            Error::InvalidPasswordResetToken => "Invalid password reset token".to_string(),
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = self.http_status();
        let message = self.message();
        let body = format!(r#"{{"error":"{}"}}"#, message);
        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.len(), std::io::Cursor::new(body))
            .ok()
    }
}

impl From<surrealdb::Error> for Error {
    fn from(error: surrealdb::Error) -> Self {
        log_error!("DB error: {:?}", error);
        Self::Db(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        log_error!("Reqwest error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        log_error!("JSON parsing error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        log_error!("IO error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        log_error!("Env var error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        log_error!("Parse int error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<lettre::transport::smtp::Error> for Error {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        log_error!("SMTP error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<lettre::error::Error> for Error {
    fn from(error: lettre::error::Error) -> Self {
        log_error!("Lettre error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<lettre::address::AddressError> for Error {
    fn from(error: lettre::address::AddressError) -> Self {
        log_error!("Address error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        log_error!("Generic boxed error: {:?}", error);
        Self::Internal(error.to_string())
    }
}

// To add support for a new error type:
// impl From<your_crate::Error> for Error {
//     fn from(error: your_crate::Error) -> Self {
//         log_error!("...: {:?}", error);
//         Self::Internal(error.to_string())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;

    #[test]
    fn unauthorized_maps_to_401() {
        assert_eq!(Error::Unauthorized.http_status(), Status::Unauthorized);
    }

    #[test]
    fn forbidden_maps_to_403() {
        assert_eq!(Error::Forbidden.http_status(), Status::Forbidden);
    }

    #[test]
    fn forbidden_message() {
        assert_eq!(Error::Forbidden.message(), "forbidden");
    }

    #[test]
    fn bad_request_maps_to_400() {

        assert_eq!(
            Error::BadRequest("missing field".to_string()).http_status(),
            Status::BadRequest
        );
    }

    #[test]
    fn internal_maps_to_500() {
        assert_eq!(
            Error::Internal("boom".to_string()).http_status(),
            Status::InternalServerError
        );
    }

    #[test]
    fn bad_gateway_maps_to_502() {
        assert_eq!(Error::BadGateway.http_status(), Status::BadGateway);
    }

    #[test]
    fn service_unavailable_maps_to_503() {
        assert_eq!(
            Error::ServiceUnavailable.http_status(),
            Status::ServiceUnavailable
        );
    }

    #[test]
    fn not_found_maps_to_404() {
        assert_eq!(Error::NotFound.http_status(), Status::NotFound);
    }

    #[test]
    fn payload_too_large_maps_to_413() {
        assert_eq!(Error::PayloadTooLarge.http_status(), Status::PayloadTooLarge);
    }

    #[test]
    fn password_reset_token_not_found_maps_to_404() {
        assert_eq!(
            Error::PasswordResetTokenNotFound.http_status(),
            Status::NotFound
        );
    }

    #[test]
    fn password_reset_token_expired_maps_to_400() {
        assert_eq!(
            Error::PasswordResetTokenExpired.http_status(),
            Status::BadRequest
        );
    }

    #[test]
    fn password_reset_token_used_maps_to_400() {
        assert_eq!(
            Error::PasswordResetTokenUsed.http_status(),
            Status::BadRequest
        );
    }

    #[test]
    fn invalid_password_reset_token_maps_to_400() {
        assert_eq!(
            Error::InvalidPasswordResetToken.http_status(),
            Status::BadRequest
        );
    }

    #[test]
    fn unauthorized_message() {
        assert_eq!(Error::Unauthorized.message(), "unauthorized");
    }

    #[test]
    fn bad_request_message_contains_provided_string() {
        let msg = "email is required".to_string();
        assert_eq!(Error::BadRequest(msg.clone()).message(), msg);
    }

    #[test]
    fn not_found_message() {
        assert_eq!(Error::NotFound.message(), "Not Found");
    }

    #[test]
    fn from_serde_json_error_produces_internal() {
        let json_err: serde_json::Error =
            serde_json::from_str::<serde_json::Value>("not json").unwrap_err();
        let err = Error::from(json_err);
        assert!(matches!(err, Error::Internal(_)));
    }

    #[test]
    fn from_io_error_produces_internal() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let err = Error::from(io_err);
        assert!(matches!(err, Error::Internal(_)));
    }

    #[test]
    fn from_env_var_error_produces_internal() {
        let env_err = std::env::var("THIS_DOES_NOT_EXIST_XYZ").unwrap_err();
        let err = Error::from(env_err);
        assert!(matches!(err, Error::Internal(_)));
    }

    #[test]
    fn from_parse_int_error_produces_internal() {
        let parse_err = "not_a_number".parse::<i64>().unwrap_err();
        let err = Error::from(parse_err);
        assert!(matches!(err, Error::Internal(_)));
    }
}
