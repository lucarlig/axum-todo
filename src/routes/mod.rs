use axum::{routing::get, Router};

mod hello_world;

pub fn create_router() -> Router {
    Router::new().route("/hello_world", get(hello_world::handler))
}
