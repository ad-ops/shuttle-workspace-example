# Shuttle Workspace
Want to try to see how it works building multiple services with [shuttle.rs](https://www.shuttle.rs) in one workspace.

I want to understand:
- Can you link to a shared crate between multiple shuttle apps?
- If you want to call different shuttle apps, how do you authenticate?
- Is it easy to run it locally?

## Setup
You need to install:
- Rust
- Docker
- cargo-shuttle: `cargo install cargo-shuttle`

## Add Service
Adding more shuttle apps can be done with.

```sh
cargo shuttle init --template axum --name my-service ./crates/my-service
```

## Run
```sh
cargo shuttle run
```

Will run all shuttle crates on different ports