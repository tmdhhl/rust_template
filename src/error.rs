use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("转链错误: {0}")]
    Translate(#[from] TranslateError),

    #[error("服务器内部错误: {0}")]
    Internal(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 转链相关错误
#[derive(Error, Debug)]
pub enum TranslateError {
    #[error("HTTP请求失败: {0}")]
    Request(String),

    #[error("服务器内部错误: {0}")]
    Internal(String),

    #[error("平台不支持: {0}")]
    UnsupportedPlatform(String),
}

/// 转链结果类型
pub type TranslateResult<T> = Result<T, TranslateError>;

/// 应用结果类型
pub type AppResult<T> = Result<T, AppError>;

/// 将 reqwest::Error 转换为 TranslateError
impl From<reqwest::Error> for TranslateError {
    fn from(err: reqwest::Error) -> Self {
        TranslateError::Request(err.to_string())
    }
}

/// 实现 AppError 到 HTTP 响应的转换
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Translate(TranslateError::UnsupportedPlatform(_)) => {
                (StatusCode::NOT_IMPLEMENTED, self.to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, message).into_response()
    }
}
