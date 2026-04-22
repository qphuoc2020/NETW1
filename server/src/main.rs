use axum::{
    extract::{Path, Query},
        response::{Html, IntoResponse},
            routing::get,
                Router,

};

use serde::Deserialize;
    use tower_http::services::ServeDir;
        use sqlx::sqlite::SqlitePool;
            use std::net::SocketAddr;
                use chrono::Local;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:hutech.db?mode=rwc")
        .await
        .expect("Không thể kết nối database");



                    let app = Router::new()
                         .route("/hello", get(handler_hello))
                                  .route("/hello/{name}", get(handler_hello_path)) 
        .route("/api/time", get(handler_time))
        
            .fallback_service(ServeDir::new("web")) 
        
                .with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
                    axum::serve(listener, app).await.unwrap();
}



#[derive(Deserialize)]
        struct HelloParams {
            name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
         format!("Hello, {}!", name)
}

async fn handler_hello_path(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("<h1>Chào <strong>{name}</strong> từ địa chỉ Path!</h1>"))
}

async fn handler_time() -> String {
    let now = Local::now();
            format!("Hệ thống HUTECH - Thời gian: {}", now.format("%d/%m/%Y %H:%M:%S"))
}