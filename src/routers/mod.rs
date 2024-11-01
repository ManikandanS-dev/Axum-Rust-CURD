mod body_mirror;
mod hello_world;
mod mirror_body_json;

use axum::{
    routing::{get, post},
    Router,
};
use body_mirror::body_mirror;
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;

pub fn routers() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirorr_body", post(body_mirror))
        .route("/mirror_body_json", post(mirror_body_json))
}
