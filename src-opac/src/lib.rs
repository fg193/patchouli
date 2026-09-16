mod error;
mod models;
pub mod providers;

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    fn provider(&self) -> ProviderId;
    async fn search(
        &self,
        client: &reqwest::Client,
        query: &str,
        page: u32,
        page_size: u32,
    ) -> Result<SearchResponse, ApiError>;
}

pub struct Opac {
    providers: Vec<Box<dyn Provider>>,
}

impl Opac {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn register(&mut self, provider: Box<dyn Provider>) {
        self.providers.push(provider);
    }

    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse, ApiError> {
        let query = request.query.trim();
        if query.is_empty() {
            return Err(ApiError::invalid_request("请输入搜索关键词"));
        }
        if query.chars().count() > 200 {
            return Err(ApiError::invalid_request("搜索关键词不能超过 200 个字符"));
        }
        let page = request.page.unwrap_or(1).max(1);
        let page_size = request.page_size.unwrap_or(20).clamp(1, 50);
        let client = reqwest::Client::builder()
            .user_agent("Patchouli/0.1 (desktop library search)")
            .timeout(std::time::Duration::from_secs(20))
            .build()?;
        self.providers
            .iter()
            .find(|p| p.provider() == request.provider)
            .ok_or(ApiError::invalid_request("不支持的检索引擎"))?
            .search(&client, query, page, page_size)
            .await
    }
}

pub use error::ApiError;
pub use models::{BookResult, ProviderId, SearchRequest, SearchResponse};
