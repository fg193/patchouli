use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    // INVALID_REQUEST: 请求参数无效
    #[error("{message}")]
    InvalidRequest { message: String },
    // HTTP: 上游服务返回 HTTP 错误状态
    #[error("HTTP {status}")]
    Http { status: u16 },
    // NETWORK_ERROR: 网络请求失败
    #[error("网络请求失败：{0}")]
    Network(#[from] reqwest::Error),
    // INVALID_RESPONSE: 上游响应无法解析
    #[error("无法解析查询结果：{response}")]
    InvalidResponse { response: String },
    // BUSINESS_ERROR: 上游服务返回业务错误码和消息
    #[error("{message}")]
    Business { code: u64, message: String },
}

impl ApiError {
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest {
            message: message.into(),
        }
    }

    pub fn http(status: reqwest::StatusCode) -> Self {
        Self::Http {
            status: status.as_u16(),
        }
    }

    pub fn invalid_response(response: &serde_json::Value) -> Self {
        Self::InvalidResponse {
            response: response.to_string(),
        }
    }
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let code = match self {
            Self::InvalidRequest { .. } => "INVALID_REQUEST",
            Self::Http { .. } => "HTTP_ERROR",
            Self::Network(_) => "NETWORK_ERROR",
            Self::InvalidResponse { .. } => "INVALID_RESPONSE",
            Self::Business { .. } => "BUSINESS_ERROR",
        };
        let mut state = serializer.serialize_struct("ApiError", 2)?;
        state.serialize_field("code", code)?;
        state.serialize_field("message", &self.to_string())?;
        if let Self::InvalidResponse { response } = self {
            state.serialize_field("response", response)?;
        }
        if let Self::Business {
            code: business_code,
            ..
        } = self
        {
            state.serialize_field("businessCode", business_code)?;
        }
        state.end()
    }
}
