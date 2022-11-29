use axum::Json;

use crate::common::{Response, Result};

pub async fn usage<'a>() -> Result<Json<Response<Vec<&'a str>>>> {
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