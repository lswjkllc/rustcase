use super::error::AppError;

/// 定义自己的 Result
pub type Result<T> = std::result::Result<T, AppError>;