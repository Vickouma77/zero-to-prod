use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Read the configuration file
    let configuration = get_configurations().expect("Failed to read configurations");

    // Bind the address
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Bind the listener
    let listener = TcpListener::bind(address)?;

    // Run the server
    run(listener)?.await
}
