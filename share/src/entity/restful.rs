use axum::response::IntoResponse;
use axum::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

// 标准返回值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysResult<T, E> {
    pub code: String,
    pub message: String,
    pub data: Option<T>,
    pub error: Option<E>,
    pub succeed: bool,
}
impl<T: serde::Serialize, E: serde::Serialize> IntoResponse for SysResult<T, E> {
    fn into_response(self) -> axum::response::Response {
        let val = json!(self); // 将 ApiResult 转换为 JSON 格式
        Json(val).into_response() // 将 JSON 响应转换为 Axum 的响应格式
    }
}

impl<T, E> SysResult<T, E> {
    // 成功
    pub fn ok(data: T) -> Self {
        Self {
            code: "A0000".to_string(),
            message: "success".to_string(),
            data: Some(data),
            error: None,
            succeed: true,
        }
    }
    // 警告
    pub fn warn(error: E) -> Self {
        Self {
            code: "W0000".to_string(),
            message: "warning".to_string(),
            data: None,
            error: Some(error),
            succeed: false,
        }
    }
    // 失败
    pub fn fail(error: E) -> Self {
        Self {
            code: "F0000".to_string(),
            message: "error".to_string(),
            data: None,
            error: Some(error),
            succeed: false,
        }
    }
}