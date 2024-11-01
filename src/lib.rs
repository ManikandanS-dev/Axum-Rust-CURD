use routers::routers;

mod routers;

pub async fn run_app() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routers()).await.unwrap();
}
