use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{message}")]
    InvalidRequest { message: String },
    #[error("{provider} 暂时无法访问（HTTP {status}）")]
    Upstream { provider: &'static str, status: u16 },
    #[error("网络请求失败：{0}")]
    Network(#[from] reqwest::Error),
    #[error("{provider} 返回了无法识别的数据")]
    InvalidResponse { provider: &'static str },
}

impl ApiError {
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest {
            message: message.into(),
        }
    }

    pub fn upstream(provider: &'static str, status: reqwest::StatusCode) -> Self {
        Self::Upstream {
            provider,
            status: status.as_u16(),
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
            Self::Upstream { .. } => "UPSTREAM_ERROR",
            Self::Network(_) => "NETWORK_ERROR",
            Self::InvalidResponse { .. } => "INVALID_RESPONSE",
        };
        let mut state = serializer.serialize_struct("ApiError", 2)?;
        state.serialize_field("code", code)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}
