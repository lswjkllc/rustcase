use std::sync::Arc;

use axum::{Json, extract::Extension};

use crate::common::{Response, Result, Container};

pub async fn usage<'a>(Extension(container): Extension<Arc<Container>>) -> Result<Json<Response<Vec<&'a str>>>> {
    tracing::info!("{:?}", container.config);

    let data = r#"
        GET /help -- 接口简介
        GET /randnum -- 获取随机数标签
    "#;
    let data: Vec<&str> = data
        .split('\n')
        .into_iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    let data = Response::ok(data);
    Ok(Json(data))
}