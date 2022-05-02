## Getting started

### Installing rust toolchain

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 

&nbsp;

    rustup toolchain install nightly --allow-downgrade

### Database set up

#### Installing sqlx-cli

    cargo install --locked sqlx-cli --no-default-features --features postgres```

#### Run db migrations locally

    SKIP_DOCKER=true ./scripts/init_db.sh


### Deployment

The application is hosted on Digital Ocean's Apps platform. 

#### Installing Digital Oceans CLI

https://www.digitalocean.com/docs/apis-clis/doctl/how-to/install/

#### Creating an app on Digital Ocean

    doctl apps create --spec spec.yaml

#### List apps on Digital Ocean

    doctl apps list

#### Updating app on Digital Ocean

    doctl apps update YOUR-APP-ID --spec=spec.yaml

#### Running db migrations on production
    DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run

### Continuous Integration

#### Tests
Running unit and integration tests

    cargo test


Running single test with logs
    
    export RUST_LOG="sqlx=error,info"
    export TEST_LOG=enabled
    TEST_LOG=true cargo test health_check_works | bunyan


#### Code coverage

Install tarpaulin locally  with

    cargo install cargo-tarpaulin

Run code coverage report locally with

    cargo tarpaulin --ignore-tests

#### Linting

Installing clippy locally

    rustup component add clippy

Running clippy on project

    cargo clippy

#### Formatting

Installing rustfmt locally

    rustup component add rustfmt

Running rustfmt on project

    cargo fmt

#### Security vulnerabilities

Installing cargo audit locally

    cargo install cargo-audit

Scanning dependency tree with cargo audit

    cargo audit

#### Cargo watch

    cargo install cargo-watch
    cargo watch -x check
    cargo watch -x check -x test -x "run | bunyan"

### Sqlx 
    sqlx migrate run
    SKIP_DOCKER=true ./scripts/init_db.sh