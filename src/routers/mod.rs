mod body_mirror;
mod hello_world;
mod mirror_body_json;
mod path_variable;
mod queary_parameter;

use axum::{
    routing::{get, post},
    Router,
};
use body_mirror::body_mirror;
use hello_world::hello_world;
use mirror_body_json::mirror_body_json;
use path_variable::path_variable;
use queary_parameter::queary_parameter;

pub fn routers() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/mirorr_body", post(body_mirror))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variable/:id", get(path_variable))
        // .route("/path_variable/0", get(path_default))
        .route("/queary_parameter", get(queary_parameter))
}
