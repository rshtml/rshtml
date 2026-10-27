#![allow(dead_code)]

mod vmacro;

use actix_web::Responder;
use axum::routing::get;
use axum::{Router, response::IntoResponse};
use chrono::{DateTime, Datelike, Utc};
use rshtml::{View, functions::*, v};
use tower_http::services::ServeDir;

#[derive(View)]
#[view(path = "views/index.rs.html")]
struct IndexPage {
    pub title: String,
    pub email: String,
    pub footer: bool,
    pub navbar_sections: Vec<(&'static str, &'static str)>,
    pub home_time: DateTime<Utc>,
}

fn main() {
    axum_server();
    // actix_server();
}

#[tokio::main]
async fn axum_server() {
    let app = Router::new()
        .route("/", get(axum_index))
        .route("/rshtml_file", get(axum_index_rshtml_file))
        .nest_service("/css", ServeDir::new("views/css"))
        .nest_service("/img", ServeDir::new("views/img"))
        .nest_service("/js", ServeDir::new("views/js"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("✅ Server started at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

/// This method works with the `v` macro.
async fn axum_index() -> impl IntoResponse {
    v!({ vmacro::index::index() })
}

/// This method works with `View` macro and `.rs.html` files.
async fn axum_index_rshtml_file() -> impl IntoResponse {
    IndexPage {
        title: "RsHtml Web Page".to_string(),
        email: "contact@rshtml.com".to_string(),
        footer: true,
        navbar_sections: vec![
            ("#tm-section-1", "Home"),
            ("#tm-section-2", "Services"),
            ("#tm-section-3", "About"),
            ("#tm-section-4", "Contact"),
        ],
        home_time: Utc::now(),
    }
}

#[actix_web::main]
async fn actix_server() {
    use actix_files::Files;
    use actix_web::{App, HttpServer, web};

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(actix_index))
            .route("/rshtml_file", web::get().to(actix_index_rshtml_file))
            .service(Files::new("/css", "views/css"))
            .service(Files::new("/img", "views/img"))
            .service(Files::new("/js", "views/js"))
    })
    .bind("0.0.0.0:8080")
    .unwrap();

    println!("✅ Server started at http://0.0.0.0:8080");

    server.run().await.unwrap();
}

/// This method works with the `v` macro.
async fn actix_index() -> impl Responder {
    v!({ vmacro::index::index() })
}

/// This method works with `View` macro and `.rs.html` files.
async fn actix_index_rshtml_file() -> impl Responder {
    IndexPage {
        title: "RsHtml Web Page".to_string(),
        email: "contact@rshtml.com".to_string(),
        footer: true,
        navbar_sections: vec![
            ("#tm-section-1", "Home"),
            ("#tm-section-2", "Services"),
            ("#tm-section-3", "About"),
            ("#tm-section-4", "Contact"),
        ],
        home_time: Utc::now(),
    }
}
