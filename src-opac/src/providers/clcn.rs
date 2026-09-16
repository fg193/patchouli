use serde_json::Value;
use sha1::{Digest, Sha1};

use super::super::{
    error::ApiError,
    models::{optional_string, string_list, BookResult, SearchResponse},
};
use super::super::{EngineId, Provider};

pub struct ClcnProvider;

#[async_trait::async_trait]
impl Provider for ClcnProvider {
    fn engine(&self) -> EngineId {
        EngineId::Clcn
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

const ENDPOINT: &str = "http://110.43.204.231/app/opac/books_new";
const SIGNING_SECRET: &str = "clcn7BiqSgJfnz1z";

fn search_params(query: &str, page: u32, page_size: u32) -> Vec<(&'static str, String)> {
    // The service distinguishes omitted filters from explicitly empty filters.
    // `start` is one-based, unlike the offsets used by the other providers.
    let start = (page - 1) * page_size + 1;
    vec![
        ("query", query.to_owned()),
        ("find_type", "creator".into()),
        ("start", start.to_string()),
        ("length", page_size.to_string()),
        ("facettype", String::new()),
        ("cdate", String::new()),
        ("loc", String::new()),
        ("library", "all".into()),
    ]
}

pub async fn search(
    client: &reqwest::Client,
    query: &str,
    page: u32,
    page_size: u32,
) -> Result<SearchResponse, ApiError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string();
    let sign = hex::encode(Sha1::digest(
        format!("{timestamp}{SIGNING_SECRET}").as_bytes(),
    ));
    let params = search_params(query, page, page_size);
    let start = (page - 1) * page_size + 1;
    let response = client
        .get(ENDPOINT)
        .header("timestamp", &timestamp)
        .header("sign", sign)
        .query(&params)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(ApiError::upstream("首都图书馆", response.status()));
    }
    let payload: Value = response.json().await?;
    let data = payload.get("data").unwrap_or(&payload);
    let rows = data
        .get("rows")
        .or_else(|| data.get("list"))
        .or_else(|| data.get("content"))
        .and_then(Value::as_array)
        .ok_or(ApiError::InvalidResponse {
            provider: "首都图书馆",
        })?;
    let items = rows
        .iter()
        .enumerate()
        .map(|(index, book)| {
            let id = optional_string(book, &["ilsapiid", "ilsApiId", "id", "recordId"])
                .unwrap_or_else(|| format!("clcn-{start}-{index}"));
            let detail_url = optional_string(book, &["url", "detailUrl"]);
            BookResult {
                id,
                title: optional_string(book, &["title", "name", "bookName"])
                    .unwrap_or_else(|| "未命名书目".into()),
                subtitle: optional_string(book, &["subtitle", "subTitle"]),
                authors: string_list(book, &["creator", "author", "authors"]),
                publisher: optional_string(book, &["publisher", "publishHouse", "lsr03"]),
                published_at: optional_string(
                    book,
                    &["creationdate", "cdate", "publishYear", "pubdate"],
                ),
                cover_url: optional_string(book, &["cover", "coverUrl", "image", "img"]),
                isbn: optional_string(book, &["identifier", "isbn", "ISBN"])
                    .map(|isbn| isbn.trim_start_matches("ISBN").trim().to_owned()),
                summary: optional_string(book, &["description", "desc", "summary"]),
                rating: None,
                detail_url,
                source: "clcn",
            }
        })
        .collect();
    Ok(SearchResponse {
        items,
        total: data
            .get("total")
            .and_then(Value::as_u64)
            .unwrap_or(rows.len() as u64),
        page,
        page_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_required_empty_filters_and_one_based_start() {
        assert_eq!(
            search_params("Donna Wheeler", 1, 9),
            vec![
                ("query", "Donna Wheeler".into()),
                ("find_type", "creator".into()),
                ("start", "1".into()),
                ("length", "9".into()),
                ("facettype", String::new()),
                ("cdate", String::new()),
                ("loc", String::new()),
                ("library", "all".into()),
            ]
        );
    }
}
