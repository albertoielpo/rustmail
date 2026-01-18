//! Rustmail
//!
//! # Author
//! Alberto Ielpo
//!
//! # License
//! MIT

use actix_web::{
    App, HttpServer,
    middleware::{Logger, NormalizePath, TrailingSlash},
    web,
};
use actix_web_lab::middleware::CatchPanic;
use log::{debug, info};
use rustmail::{
    send,
    settings::{build_server_bind, build_smtp_config, init_logger},
};

/// Application entry point.
/// Initializes the Actix-web server and starts listening for HTTP requests on 0.0.0.0:3333.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    let server_bind = build_server_bind();
    let smtp_config = build_smtp_config();

    debug!(
        "Server bind: address {} port {} workers {}",
        server_bind.addr, server_bind.port, server_bind.workers
    );
    debug!(
        "SMTP config: host {} port {} use_tls {}",
        smtp_config.host, smtp_config.port, smtp_config.use_tls
    );

    // Create HTTP server with middleware and routes
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(smtp_config.clone()))
            .wrap(NormalizePath::new(TrailingSlash::Trim)) // Normalize URL paths
            .wrap(CatchPanic::default()) // Catch panics (must be before Logger)
            .wrap(Logger::default()) // Request logging middleware
            .configure(send::send_controller::config)
    })
    .workers(server_bind.workers);

    info!("HTTP mode enabled");

    // Start HTTP server
    server
        .bind((server_bind.addr.as_str(), server_bind.port))?
        .run()
        .await
}
