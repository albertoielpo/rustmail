//! Application settings and configuration module
//!
//! This module handles all configuration loading from environment variables,
//! logging initialization, and provides common response structures.

use std::env;

use actix_web::{HttpResponse, error::InternalError};

use serde::Serialize;

// Default configuration constants
const DEFAULT_PORT: u16 = 3333;
const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_SMTP_HOST: &str = "localhost";
const DEFAULT_SMTP_PORT: u16 = 25;

/// Server binding configuration
///
/// Contains the network address, port, and worker threads configuration
/// for the HTTP server.
pub struct ServerBind {
    /// Server bind address (e.g., "0.0.0.0")
    pub addr: String,

    /// Server listening port
    pub port: u16,

    /// Number of worker threads
    pub workers: usize,
}

/// SMTP server configuration
///
/// Contains all settings required to connect and authenticate
/// with an SMTP server for sending emails.
#[derive(Clone)]
pub struct SmtpConfig {
    /// SMTP server hostname or IP address
    pub host: String,

    /// SMTP server port (typically 25, 587, or 465)
    pub port: u16,

    /// Optional SMTP authentication username
    pub username: Option<String>,

    /// Optional SMTP authentication password
    pub password: Option<String>,

    /// Whether to use TLS/STARTTLS for secure connection
    pub use_tls: bool,
}

/// API response status enumeration
///
/// Represents the status of an API operation using JSend-style conventions.
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Successful operation
    Ok,

    /// Failed operation (client error)
    Fail,

    /// Error operation (server error)
    Error,
}

/// Standard JSON response structure
///
/// Provides a consistent response format for all API endpoints.
#[derive(Serialize)]
pub struct RustMailRes {
    /// Response status (ok, fail, or error)
    pub status: Status,

    /// Human-readable message describing the result
    pub message: String,
}

/// Initializes the logger with environment variable configuration
///
/// Uses `RUST_LOG` environment variable, defaults to `debug` level.
/// This should be called once at application startup.
pub fn init_logger() {
    // Initialize the env_logger with default debug level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
}

/// Builds server bind configuration from environment variables
///
/// # Environment Variables
/// - `BIND_ADDR` - Server bind address (default: 0.0.0.0)
/// - `BIND_PORT` - Server port (default: 3333)
/// - `BIND_WORKERS` - Number of worker threads (default: number of CPU cores)
///
/// # Returns
/// A `ServerBind` struct containing the server configuration
pub fn build_server_bind() -> ServerBind {
    // Read server bind address from environment or use default
    let addr = match env::var("BIND_ADDR") {
        Ok(v) => v,
        Err(_) => DEFAULT_ADDRESS.into(),
    };

    // Read server port from environment or use default
    let port = match env::var("BIND_PORT") {
        Ok(v) => v.parse::<u16>().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    // Get the number of CPU threads, fallback to 1 if unavailable
    let default_workers = std::thread::available_parallelism()
        .map(|w| w.get())
        .unwrap_or(1);

    // Use environment variable for workers or fallback to CPU count
    let workers = env::var("BIND_WORKERS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default_workers);

    ServerBind {
        addr,
        port,
        workers,
    }
}

/// Builds SMTP configuration from environment variables
///
/// # Environment Variables
/// - `SMTP_HOST` - SMTP server hostname or IP (default: localhost)
/// - `SMTP_PORT` - SMTP server port (default: 25)
/// - `SMTP_USERNAME` - SMTP authentication username (optional)
/// - `SMTP_PASSWORD` - SMTP authentication password (optional)
/// - `SMTP_USE_TLS` - Use TLS/STARTTLS (default: false for port 25, true for others)
///
/// # Returns
/// An `SmtpConfig` struct containing the SMTP configuration
///
/// # Notes
/// TLS is automatically enabled for all ports except 25 (plain SMTP) unless
/// explicitly overridden by the `SMTP_USE_TLS` environment variable.
pub fn build_smtp_config() -> SmtpConfig {
    // Read SMTP host from environment or use default
    let host = env::var("SMTP_HOST").unwrap_or_else(|_| DEFAULT_SMTP_HOST.into());

    // Read SMTP port from environment or use default
    let port = env::var("SMTP_PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(DEFAULT_SMTP_PORT);

    // Read optional authentication credentials
    let username = env::var("SMTP_USERNAME").ok();
    let password = env::var("SMTP_PASSWORD").ok();

    // Automatically enable TLS for all ports except 25 (plain SMTP)
    let default_use_tls = port != 25;
    let use_tls = env::var("SMTP_USE_TLS")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(default_use_tls);

    SmtpConfig {
        host,
        port,
        username,
        password,
        use_tls,
    }
}

/// Converts any error into an Actix-web JSON error response
///
/// This helper function wraps errors in a consistent JSON format with HTTP 500 status.
/// All errors are returned as `application/json` instead of `text/plain`.
///
/// # Arguments
/// * `err` - Any error type that implements `Display`
///
/// # Returns
/// An `actix_web::Error` that produces a JSON response with the error message
///
/// # Response Format
/// ```json
/// {
///   "status": "error",
///   "message": "error description here"
/// }
/// ```
pub fn json_error<E: std::fmt::Display>(err: E) -> actix_web::Error {
    let error_response = RustMailRes {
        status: Status::Error,
        message: err.to_string(),
    };
    InternalError::from_response(
        err.to_string(),
        HttpResponse::InternalServerError().json(error_response),
    )
    .into()
}
