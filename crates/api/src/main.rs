use axum::{routing::get, Router, extract::State};
use util::add;
use shuttle_persist::PersistInstance;

async fn hello_world() -> String {
    let answer = add(40, 2);
    format!("Hello, world! The answer is {answer}")
}

async fn count(state: State<MyState>) -> String {
    let persist = &state.persist;
    let key = "count";
    let count = persist.load::<u32>(key).unwrap_or_default();
    let persist_result = persist.save(key, count + 1);
    match persist_result {
        Ok(_) => format!("current count is {count}"),
        Err(e) => format!("Could not persist new count! Error: {e}"),
    }
}

async fn ask_other_service() -> String {
    let response = reqwest::get("http://localhost:3001").await;
    if let Ok(response) = response {
        let text = response.text().await;
        if let Ok(text) = text {
            return format!("The other service responded: {text}")
        }
    }

    "Ooops! The other service did not respond correctly!".to_string()
}

#[derive(Clone)]
struct MyState {
    persist: PersistInstance,
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> shuttle_axum::ShuttleAxum {
    let state = MyState {
        persist,
    };
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/count", get(count))
        .route("/other", get(ask_other_service))
        .with_state(state);

    Ok(router.into())
}
