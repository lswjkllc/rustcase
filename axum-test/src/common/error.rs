
use std::error;

use axum::{Json, response::IntoResponse};

use super::response::Response;

/// 错误类型
pub enum AppErrorType {
    /// 无错误
    Ok,
    /// 数据库错误
    DbType,
    /// 未发现
    NotFound,
}

/// 应用错误
pub struct AppError {
    /// 错误信息
    pub message: Option<String>,
    /// 错误原因（上一级的错误）
    pub cause: Option<String>,
    /// 错误类型
    pub error_type: AppErrorType,
}

impl AppError {
    fn code(&self) -> i32 {
        match self.error_type {
            AppErrorType::Ok => 0,
            AppErrorType::DbType => 1,
            AppErrorType::NotFound => 2,
        }
    }

    fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type: error_type,
        }
    }

    fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type: error_type,
        }
    }

    /// 数据库错误
    pub fn db_err(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::DbType)
    }

    /// 未找到
    pub fn not_found() -> Self {
        Self::from_str("不存在的记录", AppErrorType::NotFound)
    }
}

/// 为应用错误实现 IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = (&self).code();
        let msg = match self.message {
            Some(msg) => msg,
            None => "".to_string(),
        };

        let resp: Response<()> = Response::err(code, msg);
        Json(resp).into_response()
    }
}