use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service}, 
    Router,
};
use chrono::Local;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile}; 

#[tokio::main]
async fn main() {
    // Khởi tạo các nhóm Routes xử lý logic
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/{name}", get(handler_hello_path));

    let routes_api = Router::new()
        .route("/api/time", get(handler_time));

    // Tổng hợp tất cả tính năng
    let app = Router::new()
        .merge(routes_hello)
        .merge(routes_api)
        // Khi vào trang chủ (/) thì trả về file index.html
        .route("/", get_service(ServeFile::new("web/index.html")))
        // Cho phép truy cập hình ảnh qua đường dẫn /web/hutech-logo.png
        .nest_service("/web", ServeDir::new("web"));
        
    // Chạy Server 
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> SERVER NETW1 ĐANG CHẠY TẠI: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// CÁC HÀM XỬ LÝ (HANDLERS)

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
    format!("Ngày giờ hiện tại: {}", now.format("%d/%m/%Y %H:%M:%S"))
}