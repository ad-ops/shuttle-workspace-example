use axum::{routing::get, Router, extract::State};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn spill_secret(state: State<MyState>) -> String {
    let secret = state.secrets.get("secret");
    match secret {
        Some(secret) => format!("My secret is '{secret}'. Don't tell anyone!"),
        None => format!("There is no secret..."),
    }
}

#[derive(sqlx::FromRow, Debug)]
struct Person {
    name: String,
    age: i16,
}

async fn get_person(state: State<MyState>) -> String {
    let db = &state.db;
    let result = sqlx::query_as::<_, Person>(r#"SELECT name, age FROM persons"#)
        .fetch_one(db)
        .await;
    match result {
        Ok(p) => format!("The person is named {} and is {} old.", p.name, p.age),
        Err(e) => format!("Could not fetch any persons. Error: {e}"),
    }
}

#[derive(Clone)]
struct MyState {
    db: PgPool,
    secrets: SecretStore,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] postgres: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    sqlx::migrate!()
        .run(&postgres)
        .await
        .expect("Could not run DB migrations!");

    let state = MyState {
        db: postgres,
        secrets: secret_store,
    };
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/secret", get(spill_secret))
        .route("/person", get(get_person))
        .with_state(state);

    Ok(router.into())
}
