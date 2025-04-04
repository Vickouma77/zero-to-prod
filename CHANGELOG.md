## 0.2.0 (2025-04-04)

### Feat

- refactor subscriber parsing logic for improved readability and maintainability
- add quickcheck support for SubscriberEmail tests with valid email generation
- add fake crate for generating test emails and enhance SubscriberEmail tests
- update email validation to use regex and enhance validation logic
- update NewSubscriber to use SubscriberEmail type and adjust subscription handling
- add email validation to SubscriberEmail and implement tests
- enhance subscriber name validation and update subscription handling
- implement subscriber name validation and refactor subscription handling
- add validation for subscriber names in subscription route
- update Dockerfile to install sqlx-cli with rustls and postgres features
- add database configuration options for SSL and update environment variables in spec.yaml
- add serde-aux for improved deserialization and update database connection handling
- update spec.yaml to correct deployment branch and add database configuration
- update Dockerfile base image to debian:bookworm-slim and add spec.yaml for deployment configuration
- implement multi-stage Dockerfile using cargo-chef for improved build efficiency
- optimize Dockerfile for smaller image size and improved runtime performance
- implement multi-stage Dockerfile for optimized builds and add .dockerignore
- refactor configuration management to support environment-specific settings and update Dockerfile for production
- update dependencies for serde and tokio, and add Dockerfile for containerization
- add tracing-actix-web for improved logging and replace Logger middleware
- integrate secrecy crate for enhanced password handling in database settings
- enhance tracing instrumentation for subscriber functions
- add once_cell dependency and enhance tracing subscriber initialization
- enhance debug step in audit workflow to check for deny.toml existence
- add debug step to audit workflow for file existence check
- migrate deny.toml to .cargo directory for advisory management
- add deny.toml to ignore specific Rust advisories
- update dependencies and enhance telemetry logging
- enhance logging for new subscriber to capture both name and email
- add logging for subscriber endppoint
- add env_logger for logging and integrate with Actix web server
- add SQLx query for inserting subscriptions and regenerate query cache in CI
- add SQLX query for inserting subscriptions into PostgreSQL
- set fetch-depth to 0 in GitHub Actions workflow for complete history
- add Redis service to GitHub Actions workflow and improve toolchain installation steps
- enable SQLX offline mode for test execution in GitHub Actions
- add SQLX offline mode to test environment and create subscription query
- enable SQLX offline mode in GitHub Actions workflow
- add database creation and migration functionality
- integrate PostgreSQL database connection and subscription handling
- implement configuration management, routing, and subscription handling
- add database migration for subscriptions table and initialize database script
- add subscription endpoint and implement form data handling
- implement health check functionality, integration testings and update project structure
- add health check endpoint and update dependencies

### Fix

- add additional advisory to ignore list in cargo deny configuration
- correct formatting in subscriber name validation error handling
- update database size in spec.yaml for better resource allocation
- use connect_lazy for database connection to improve startup performance
- update dependencies and add additional advisory to ignore list
- correct command syntax for cargo deny in audit workflow
- update subscriber insertion to be asynchronous and clean up formatting
- correct command syntax for cargo deny in audit workflow
- update audit workflow to use custom deny.toml configuration
- correct indentation in error logging for subscription query
- correct formatting in subscribe function response handling
- disable SQLX offline mode in GitHub Actions workflow for test execution
- update PostgreSQL password in GitHub Actions workflow

### Refactor

- reorder imports in NewSubscriber and subscriptions modules for consistency
- restructure domain module and move subscriber-related structs to separate files
- remove commented-out connection string methods from DatabaseSettings for cleaner code
- streamline database configuration and connection handling in health check tests
- simplify health check route definition in main.rs
