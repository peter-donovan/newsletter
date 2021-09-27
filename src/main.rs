//! `main.rs` Serves as the entrypoint to the application.
//!
//! The application is bootstrapped and launched via the [`start`] function.
//!
//! [`start`]: newsletter::server::start

use std::net::TcpListener;

use sqlx::PgPool;

use newsletter::{
    app::start,
    settings::Settings,
    telemetry::{init_subscriber, register_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = register_subscriber("newsletter", "info", std::io::stdout);
    init_subscriber(subscriber);

    // Load settings from configuration source(s). Panic on failure.
    let settings = Settings::load().expect("Failed to load configuration settings.");

    let pool = PgPool::connect(&settings.database.url())
        .await
        .expect("Failed to connect to PostgreSQL");
    let addr = &format!("127.0.0.1:{}", &settings.app.port);
    let listener = TcpListener::bind(addr)?;

    start(listener, pool)?.await
}
