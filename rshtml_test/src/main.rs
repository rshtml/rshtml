use crate::website::website;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use tower_http::services::ServeDir;

mod website;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/css", ServeDir::new("views/css"))
        .nest_service("/img", ServeDir::new("views/img"))
        .nest_service("/js", ServeDir::new("views/js"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!(
        "✅ Server started at http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let page = website();

    Html(page)
}
