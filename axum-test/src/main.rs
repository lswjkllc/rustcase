use axum::{routing::get, Router};
// use std::net::SocketAddr;
use dotenv;

mod common;
mod api;

/// 定义自己的 Result
pub type Result<T> = std::result::Result<T, common::AppError>;

#[tokio::main]
async fn main() {
    // 解析 .env 文件
    // dotenv::dotenv().ok();
    dotenv::from_path("axum-test/.env").ok();
    // 初始化配置
    let cfg = common::Config::from_env().expect("初始化配置失败");

    let app = Router::new()
        .route("/", get(api::range_randnum));
    
    let addr = &cfg.web.addr.parse().unwrap();
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
