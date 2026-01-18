//! HTTP controllers for email sending endpoints
//!
//! This module provides the HTTP handlers for health checks and email sending functionality.

use crate::send::dto::SendMailReq;
use crate::settings::{RustMailRes, SmtpConfig, Status, json_error};
use actix_web::{HttpRequest, HttpResponse, Result, get, head, post, web};
use base64::{Engine, prelude::BASE64_STANDARD};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{debug, info};

/// Performs health check and returns service status
///
/// Returns a JSON response indicating the service is up and running.
fn do_health_check() -> Result<HttpResponse> {
    let x = RustMailRes {
        status: Status::Ok,
        message: "Rust mail up".to_owned(),
    };
    Ok(HttpResponse::Ok().json(x))
}

/// GET endpoint for health check
///
/// Maps to the root path and returns the service status.
#[get("")]
async fn health_check_get() -> Result<HttpResponse> {
    do_health_check()
}

/// HEAD endpoint for health check
///
/// Maps to the root path and returns the service status headers without body.
#[head("")]
async fn health_check_head() -> Result<HttpResponse> {
    do_health_check()
}

/// POST endpoint for sending emails
///
/// Receives an email request, validates it, and sends it through the configured SMTP server.
///
/// # Arguments
/// * `req` - HTTP request containing headers for logging
/// * `body` - JSON payload containing email details (from, to, subject, text, encoding)
/// * `smtp_config` - SMTP server configuration injected by Actix
///
/// # Returns
/// * `Ok(HttpResponse)` - JSON response with success message on successful send
/// * `Err(actix_web::Error)` - JSON error response on failure (invalid email, SMTP errors, etc.)
///
/// # Encoding Support
/// * `plain` - Text is sent as-is
/// * `base64` - Text is base64 decoded before sending
#[post("send")]
async fn send(
    req: HttpRequest,
    body: web::Json<SendMailReq>,
    smtp_config: web::Data<SmtpConfig>,
) -> Result<HttpResponse> {
    let host_header = req.headers().iter().find(|x| x.0.eq("host"));
    if let Some(header) = host_header {
        info!("send request from {} {:?}", header.0, header.1);
    } else {
        info!("No host header found in the request");
    }

    let payload = body.into_inner();

    // Decode email text based on encoding type
    let mut text = payload.mail.text.to_owned();
    if payload.mail.encoding.eq("base64") {
        // Decode base64 encoded text
        let x = BASE64_STANDARD
            .decode(payload.mail.text)
            .map_err(json_error)?;
        text = String::from_utf8(x).map_err(json_error)?;
    }

    debug!("{}", text);

    let mail_from = payload.mail.from.parse().map_err(json_error)?;

    // Parse all recipients
    let mail_to: Vec<_> = payload
        .mail
        .to
        .iter()
        .map(|addr| addr.parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(json_error)?;

    // Build email with multiple recipients
    let mut email_builder = Message::builder()
        .from(mail_from)
        .subject(payload.mail.subject);

    for recipient in mail_to {
        email_builder = email_builder.to(recipient);
    }

    let email = email_builder.body(text).map_err(json_error)?;

    // Build SMTP transport with configuration
    let mailer = if smtp_config.use_tls {
        // Use relay with STARTTLS
        let mut mailer_builder = SmtpTransport::relay(&smtp_config.host)
            .map_err(json_error)?
            .port(smtp_config.port);

        // Add credentials if provided
        if let (Some(username), Some(password)) = (&smtp_config.username, &smtp_config.password) {
            let creds = Credentials::new(username.clone(), password.clone());
            mailer_builder = mailer_builder.credentials(creds);
        }

        mailer_builder.build()
    } else {
        // Use plain SMTP without TLS
        let mut mailer_builder =
            SmtpTransport::builder_dangerous(&smtp_config.host).port(smtp_config.port);

        // Add credentials if provided
        if let (Some(username), Some(password)) = (&smtp_config.username, &smtp_config.password) {
            let creds = Credentials::new(username.clone(), password.clone());
            mailer_builder = mailer_builder.credentials(creds);
        }

        mailer_builder.build()
    };

    // Send the email through SMTP
    mailer.send(&email).map_err(json_error)?;

    let message = format!("Mail sent to {}", payload.mail.to.join(", "));
    info!("{}", message);

    let x = RustMailRes {
        status: Status::Ok,
        message,
    };
    Ok(HttpResponse::Ok().json(x))
}

/// Configures the Actix-web service routes
///
/// Registers all HTTP endpoints for this module with the application.
///
/// # Arguments
/// * `cfg` - Actix-web service configuration to register routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check_get);
    cfg.service(health_check_head);
    cfg.service(send);
}
