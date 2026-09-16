use serde_json::Value;

use super::super::{
    error::ApiError,
    models::{optional_string, string_list, BookResult, SearchResponse},
};
use super::super::{EngineId, Provider};

pub struct DoubanProvider;

#[async_trait::async_trait]
impl Provider for DoubanProvider {
    fn engine(&self) -> EngineId {
        EngineId::Douban
    }
    async fn search(
        &self,
        client: &reqwest::Client,
        query: &str,
        page: u32,
        page_size: u32,
    ) -> Result<SearchResponse, ApiError> {
        search(client, query, page, page_size).await
    }
}

const ENDPOINT: &str = "https://api.douban.com/v2/book/search";
const API_KEY: &str = "0ab215a8b1977939201640fa14c66bab";

pub async fn search(
    client: &reqwest::Client,
    query: &str,
    page: u32,
    page_size: u32,
) -> Result<SearchResponse, ApiError> {
    let start = (page - 1) * page_size;
    let response = client
        .post(ENDPOINT)
        .form(&[
            ("q", query.to_owned()),
            ("apikey", API_KEY.to_owned()),
            ("start", start.to_string()),
            ("count", page_size.to_string()),
        ])
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(ApiError::upstream("豆瓣", response.status()));
    }
    let payload: Value = response.json().await?;
    let books = payload
        .get("books")
        .and_then(Value::as_array)
        .ok_or(ApiError::InvalidResponse { provider: "豆瓣" })?;
    let items = books
        .iter()
        .enumerate()
        .map(|(index, book)| {
            let id = optional_string(book, &["id", "isbn13", "isbn10"])
                .unwrap_or_else(|| format!("douban-{start}-{index}"));
            let rating = book
                .get("rating")
                .and_then(|rating| rating.get("average"))
                .and_then(|average| average.as_f64().or_else(|| average.as_str()?.parse().ok()));
            BookResult {
                id,
                title: optional_string(book, &["title"]).unwrap_or_else(|| "未命名书目".into()),
                subtitle: optional_string(book, &["alt_title", "subtitle"]),
                authors: string_list(book, &["author", "translator"]),
                publisher: optional_string(book, &["publisher"]),
                published_at: optional_string(book, &["pubdate"]),
                cover_url: optional_string(book, &["image"]).or_else(|| {
                    book.pointer("/images/large")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                }),
                isbn: optional_string(book, &["isbn13", "isbn10"]),
                summary: optional_string(book, &["summary"]),
                rating,
                detail_url: optional_string(book, &["alt"]),
                source: "douban",
            }
        })
        .collect();
    Ok(SearchResponse {
        items,
        total: payload
            .get("total")
            .and_then(Value::as_u64)
            .unwrap_or(books.len() as u64),
        page,
        page_size,
    })
}
