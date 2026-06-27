use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::Mailbox,
};
use rocket::post;
use rocket::State;
use rocket::serde::json::Json;
use std::env;
use std::sync::Arc;
use reqwest::Client;
use log::info;
use crate::error::Error;
use crate::auth::{
    AuthUser,
};

// Struct for incoming JSON payload
#[derive(serde::Deserialize)]
pub struct EmailPayload {
    to: String,
    subject: String,
    body: String,
}

async fn send_email_internal(payload: Json<EmailPayload>) -> Result<String, Error> {
    let from_addr: Mailbox = "brendan@bdonahue.com".parse().map_err(|e| Error::Internal(format!("Invalid from address: {}", e)))?;  // TODO: make this configurable via env
    let to_addr: Mailbox = payload.to.parse().map_err(|e| Error::Internal(format!("Invalid to address: {}", e)))?;

    // Improved: Use multipart/alternative for better deliverability (plain text + HTML)
    let plain_text = "You're invited to join the Tile App.\n\nClick here to set your password: [link]".to_string(); // simple fallback

    let email = Message::builder()
        .from(from_addr)
        .to(to_addr)
        .subject(&payload.subject)
        .multipart(
            lettre::message::MultiPart::alternative()
                .singlepart(lettre::message::SinglePart::plain(plain_text))
                .singlepart(
                    lettre::message::SinglePart::html(payload.body.clone())
                )
        )?;
    
    info!("{:?}", email);


    let smtp_user = env::var("SMTP_USER")
        .map_err(|e| Error::Internal(format!("SMTP_USER not set: {}", e)))?;

    info!("{:?}", smtp_user);

    let smtp_pass = env::var("SMTP_PASS")
        .map_err(|e| Error::Internal(format!("SMTP_PASS not set: {}", e)))?;

    info!("{:?}", smtp_pass);
        
    let creds = Credentials::new(smtp_user, smtp_pass);

    info!("{:?}", creds);

    let smtp_host = env::var("SMTP_HOST")
        .map_err(|e| Error::Internal(format!("SMTP_HOST not set: {}", e)))?;

    info!("{:?}", smtp_host);

    let smtp_port: u16 = env::var("SMTP_PORT")
        .map_err(|e| Error::Internal(format!("SMTP_PORT not set: {}", e)))?
        .parse()
        .map_err(|e| Error::Internal(format!("SMTP_PORT invalid: {}", e)))?;

    info!("{:?}", smtp_port);

    let mailer: AsyncSmtpTransport<Tokio1Executor> = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    info!("{:?}", mailer);

    mailer.send(email).await?;

    Ok("Email sent".to_string())
}

#[post("/send_email", data = "<payload>")]
pub async fn send_email(user: AuthUser, payload: Json<EmailPayload>) -> Result<String, Error> {
    if user.role != "admin" && user.role != "company" { return Err(Error::Unauthorized); }
    send_email_internal(payload).await  // delegate
}

pub async fn send_invite_email(
    to_email: &str,
    invite_token: &str,
    role: &str,
) -> Result<(), Error> {
    let frontend_url = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:1420".to_string());

    let invite_link = format!("{}/invite/{}", frontend_url, invite_token);

    let html_body = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head><meta charset="utf-8"></head>
        <body style="font-family: system-ui, -apple-system, sans-serif; background:#f8fafc; padding:40px;">
            <div style="max-width:520px; margin:0 auto; background:white; border-radius:12px; padding:40px; box-shadow:0 10px 15px -3px rgb(0 0 0 / 0.1);">
                <h1 style="color:#1e40af; margin-bottom:24px;">Welcome to Cracked Stack!</h1>
                <p style="font-size:17px; line-height:1.6; color:#334155;">
                    Hi,<br><br>
                    You've been invited to join as a <strong>{role}</strong>.
                </p>
                <p style="font-size:17px; line-height:1.6; color:#334155;">
                    Click the button below to set your password and activate your account.<br>
                    This link expires in 7 days.
                </p>
                
                <a href="{link}" 
                   style="display:inline-block; background:#2563eb; color:white; padding:14px 32px; 
                          text-decoration:none; border-radius:8px; font-weight:600; margin:28px 0 20px;">
                    Set My Password
                </a>
                
                <p style="font-size:15px; color:#64748b; word-break:break-all;">
                    Or copy this link:<br>
                    <span style="font-family:monospace;">{link}</span>
                </p>
                
                <hr style="border:none; border-top:1px solid #e2e8f0; margin:32px 0;">
                <p style="font-size:14px; color:#64748b;">
                    This invitation was sent by the Cracked Stack admin team.<br>
                    If you didn't expect this email, please ignore it.
                </p>
            </div>
        </body>
        </html>
        "#,
        role = role,
        link = invite_link
    );

    let payload = Json(EmailPayload {
        to: to_email.to_string(),
        subject: "You're invited to join Cracked Stack!".to_string(),
        body: html_body,
    });

    // Call the internal function directly (no Rocket request needed)
    send_email_internal(payload).await
        .map_err(|e| Error::Internal(format!("Failed to send invite email: {}", e)))?;

    Ok(())
}

pub async fn send_password_reset_email(
    to_email: &str,
    reset_token: &str,
) -> Result<(), Error> {
    let frontend_url = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:1420".to_string());

    let reset_link = format!("{}/reset-password/{}", frontend_url, reset_token);

    let html_body = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head><meta charset="utf-8"></head>
        <body style="font-family: system-ui, -apple-system, sans-serif; background:#f8fafc; padding:40px;">
            <div style="max-width:520px; margin:0 auto; background:white; border-radius:12px; padding:40px; box-shadow:0 10px 15px -3px rgb(0 0 0 / 0.1);">
                <h1 style="color:#1e40af; margin-bottom:24px;">Reset Your Password</h1>
                <p style="font-size:17px; line-height:1.6; color:#334155;">
                    Hi,<br><br>
                    We received a request to reset your password for your Cracked Stack account.
                </p>
                <p style="font-size:17px; line-height:1.6; color:#334155;">
                    Click the button below to reset your password.<br>
                    This link expires in 1 hour.
                </p>

                <a href="{link}"
                   style="display:inline-block; background:#2563eb; color:white; padding:14px 32px;
                          text-decoration:none; border-radius:8px; font-weight:600; margin:28px 0 20px;">
                    Reset Password
                </a>

                <p style="font-size:15px; color:#64748b; word-break:break-all;">
                    Or copy this link:<br>
                    <span style="font-family:monospace;">{link}</span>
                </p>

                <hr style="border:none; border-top:1px solid #e2e8f0; margin:32px 0;">
                <p style="font-size:14px; color:#64748b;">
                    This password reset was requested for <strong>{email}</strong>.<br>
                    If you didn't request this password reset, please ignore this email.<br>
                    Your password will remain unchanged.
                </p>
            </div>
        </body>
        </html>
        "#,
        link = reset_link,
        email = to_email
    );

    let payload = Json(EmailPayload {
        to: to_email.to_string(),
        subject: "Reset your Cracked Stack password".to_string(),
        body: html_body,
    });

    send_email_internal(payload).await
        .map_err(|e| Error::Internal(format!("Failed to send password reset email: {}", e)))?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct SmsPayload {
    to: String,
    body: String,
}

#[post("/send_sms", data = "<payload>")]
pub async fn send_sms(
    _user: AuthUser, 
    payload: Json<SmsPayload>, 
    http: &State<Arc<Client>>,
) -> Result<String, Error> {
    let http_client = &**http;

    let account_sid = env::var("TWILIO_API_KEY_SID").unwrap();
    let auth_token = env::var("TWILIO_API_KEY_SECRET").unwrap();

    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", account_sid);

    let mut form = std::collections::HashMap::new();
    form.insert("From", env::var("TWILIO_PHONE").unwrap());
    form.insert("To", payload.to.clone());
    form.insert("Body", payload.body.clone());

    let res = http_client
        .post(&url)
        .basic_auth(account_sid, Some(auth_token))
        .form(&form)
        .send()
        .await?;

    if res.status().is_success() {
        Ok("SMS sent".to_string())
    } else {
        let err_text = res.text().await.unwrap_or_default();
        Err(Error::Internal(format!("Twilio error: {}", err_text)))
    }
}