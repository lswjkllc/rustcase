use std::net::SocketAddr;

use axum::{routing::get, Router};
use tracing_subscriber::{fmt::{Layer}, layer::SubscriberExt, EnvFilter};
use dotenv;

mod common;
mod api;

#[tokio::main]
async fn main() {
    // 解析 .env 文件
    // dotenv::dotenv().ok();
    dotenv::from_path("axum-test/.env").ok();
    
    // 初始化配置
    let cfg = common::Config::from_env().expect("初始化配置失败");

    // 初始化日志
    let file_appender = tracing_appender::rolling::daily("", cfg.log.path);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(cfg.log.level.parse().unwrap()))
        .with(Layer::new().with_writer(std::io::stdout))
        .with(Layer::new().with_writer(non_blocking));
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
    // tracing_subscriber::fmt()
    //     .with_max_level(log_level)
    //     .with_writer(std::io::stdout) // 写入标准输出
    //     .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
    //     .with_ansi(false)  // 如果日志是写入文件，应将ansi的颜色输出功能关掉
    //     .init();  // 初始化并将SubScriber设置为全局SubScriber

    // 监听主机和端口号
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr: &SocketAddr = &cfg.web.addr.parse().unwrap();
    tracing::info!("服务器监听于: {}", &cfg.web.addr);

    // 路由设置
    let app = Router::new()
        .route("/randnum", get(api::range_randnum))
        .route("/help", get(api::usage));
    // 开启服务
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
