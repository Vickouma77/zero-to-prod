// This is the entry point of the application

use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Read the configuration file
    let configuration = get_configurations().expect("Failed to read configurations");

    // Connect to the database
    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    // Bind the address
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    // Bind the listener
    let listener = TcpListener::bind(address)?;

    // Run the server
    run(listener, connection)?.await
}
