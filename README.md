# Shuttle Workspace
Wanted to try to see how it works building multiple services with [shuttle.rs](https://www.shuttle.rs) in one workspace and how it feels to to use it locally.

The service `crates/other_api` will deploy locally, but not as a service on `shuttle.rs`` since it only allows for one service per workspace (see findings below).

### What is Shuttle?
It is a serverless cloud service for Rust application which also provisions infrastructure based on what is defined in the code. You can read more here: <https://docs.shuttle.rs/introduction/what-is-shuttle>

### I want to understand
- Can you link to a shared crate between multiple shuttle apps?
- Can you only use public crates?
- Does it solve authentication?
- How easy is it to run locally?

### Some findings
- Shuttle does not support multiple services in the same workspace. They seem to be working on this, but for most parts I don't think this is needed since most infrastructure is already tied to a service.
- It does work very well to use a workspace with different crates.
- There is no *infrastructure* authentication so you need to do this in the application.
- It is very easy to run locally which is great!
- Shuttle builds the crates when deploying so it can only use public crates.
- Since it uses common libraries such as Axum, SQLx it makes it very easy to migrate or to use examples and documentation for these.
- All services are deployed in London which is good for GDPR reasons. But it would be better if it was in EU.
- Everything is CLI-based which is nice.

## Setup
You need to install:
- Rust
- Docker (needed to provision the Postgres DB)
- cargo-shuttle: `cargo install cargo-shuttle`

## Run
```sh
cargo shuttle run
```

Will run all shuttle crates on different ports and provision needed resources. I could not find a way to programatically set the ports for different services, but this is expected since you can not deploy multiple services in one workspace.

## Add Service
Adding more shuttle apps can be done with.

```sh
cargo shuttle init --template axum --name my-service ./crates/my-service
```
