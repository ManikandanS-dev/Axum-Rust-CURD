use axum::http::HeaderMap;

pub async fn custom_header(headerhap: HeaderMap) -> String {
    let header = headerhap.get("mk").unwrap().to_str().unwrap().to_owned();
    header
}
