use serde::Serialize;

/// 定义自己的 Response
/// 为了以 JSON 作为响应，这个结构体以及包括data在内的所有字段必须是可序列化的，即实现了Serialize trait
#[derive(Serialize)]
pub struct Response<T: Serialize> {
    // 响应状态码。如果没有错误，该值为0
    pub code: i32,
    // 提示信息。如果没有错误，该值为""
    pub msg: String,
    // 响应的数据。如果发生错误，该值为null（Rust 里的None）
    pub data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize,
{
    // 创建一个新的响应
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self {code: code, msg: msg, data: data}
    }
    // 创建一个没有错误发生响应
    pub fn ok(data: T) -> Self {
        Self::new(0, "".to_string(), Some(data))
    }
    // 创建一个发生错误的响应
    pub fn err(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}