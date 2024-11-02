use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Parameter {
    name: String,
    age: i16,
}

pub async fn queary_parameter(Query(queary): Query<Parameter>) -> Json<Parameter> {
    Json(queary)
}
