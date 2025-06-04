use axum::Router;
use axum::response::Html;
use axum::routing::get;
use rshtml::traits::RsHtml;
use rshtml_test::{HomePage, User};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<String> {
    let users = vec![
        User {
            name: "abc".to_string(),
            age: 10,
        },
        User {
            name: "def".to_string(),
            age: 11,
        },
        User {
            name: "hjk".to_string(),
            age: 12,
        },
        User {
            name: "lmo".to_string(),
            age: 13,
        },
    ];

    let mut homepage = HomePage {
        title: "Hello".to_string(),
        content: "World".to_string(),
        card_count: 1,
        my_var: "This is my var".to_string(),
        abc: "abc".to_string(),
        def: "def".to_string(),
        inner: "inner".to_string(),
        hey: "hey".to_string(),
        is_ok: true,
        users,
    };

    let s = homepage.render().unwrap();

    Html(s)
}
