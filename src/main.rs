use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read the configuration file
    let configuration = get_configurations().expect("Failed to read configurations");

    // Connect to the database
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Bind the address
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Bind the listener
    let listener = TcpListener::bind(address)?;

    // Run the server
    run(listener, connection)?.await
}
