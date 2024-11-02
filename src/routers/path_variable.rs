use axum::extract::Path;

pub async fn path_variable(Path(id): Path<i32>) -> String {
    if id <= 0 {
        path_default().await
    } else {
        format!("you hit No: {},", { id }).to_owned()
    }
}

pub async fn path_default() -> String {
    "you hit default id<=0,".to_owned()
}
