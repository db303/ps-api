# Getting Started

This guide will help you set up your development environment and deploy the application. It covers installing the Rust toolchain, setting up the database, deploying the application on Digital Ocean, and includes sections on continuous integration practices.

## Prerequisites

Ensure you have `curl` installed on your system to fetch remote content.

### Installing Rust Toolchain

To install Rust and its toolchain, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, install the nightly toolchain:

```bash
rustup toolchain install nightly --allow-downgrade
```

## Database Setup

### Installing sqlx-cli for PostgreSQL
```bash
cargo install --locked sqlx-cli --no-default-features --features postgres
```

### Running Database Migrations Locally
To initialize the database, run:

```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```

## Deployment on Digital Ocean

### Prerequisites
Install the Digital Ocean CLI ([DOCTL](https://www.digitalocean.com/docs/apis-clis/doctl/how-to/install/))

### Creating an App on Digital Ocean

```bash
doctl apps create --spec spec.yaml
```

### Managing Apps on Digital Ocean

List existing apps:

```bash
doctl apps list
```

Update existing app
```bash
doctl apps update YOUR-APP-ID --spec=spec.yaml
```

### Running Database Migrations on Production
```bash
DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run
```

## Continuous Integration

### Running Tests
To run unit and integration tests:

```bash
cargo test
```

To run a specific test with logs:

```bash
export RUST_LOG="sqlx=error,info"
export TEST_LOG=enabled
TEST_LOG=true cargo test health_check_works | bunyan
```

### Code Coverage
Install cargo-tarpaulin:

```bash
cargo install cargo-tarpaulin
```
Generate a code coverage report:

```bash
cargo tarpaulin --ignore-tests
```

### Linting with Clippy
Install clippy:

```bash
rustup component add clippy
```

Run clippy on the project:

```bash
cargo clippy
```

### Formatting with rustfmt
Install rustfmt:

```bash
rustup component add rustfmt
```

Format the project:

```bash
cargo fmt
```

### Checking for Security Vulnerabilities
Install cargo-audit:

```bash
cargo install cargo-audit
```

Scan the dependency tree:

```bash
cargo audit
```

### Using Cargo Watch
Install and use cargo-watch for automatic checking:

```bash
cargo install cargo-watch
cargo watch -x check
```

For checking, testing, and running:

```bash
cargo watch -x check -x test -x "run | bunyan"
```

## Working with SQLx
To run migrations

```bash
sqlx migrate run
```

To initialize the database without Docker:

```bash
sqlx migrate run
SKIP_DOCKER=true ./scripts/init_db.sh
```
 