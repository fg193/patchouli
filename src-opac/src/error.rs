use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{message}")]
    InvalidRequest { message: String },
    #[error("HTTP {status}")]
    Http { status: u16 },
    #[error("网络请求失败：{0}")]
    Network(#[from] reqwest::Error),
    #[error("无法解析查询结果：{response}")]
    InvalidResponse { response: String },
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
