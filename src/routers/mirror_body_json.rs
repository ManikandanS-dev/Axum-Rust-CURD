use std::time::SystemTime;

use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorModel {
    name: String,
}

#[derive(Serialize)]
pub struct MirrorResponce {
    name: String,
    i: SystemTime,
}

pub async fn mirror_body_json(Json(body): Json<MirrorModel>) -> Json<MirrorResponce> {
    Json(MirrorResponce {
        name: body.name,
        i: SystemTime::now(),
    })
}
