use serde_json::{json, Map, Value};

use super::super::{
    error::ApiError,
    models::{optional_string, string_list, BookResult, SearchResponse},
};
use super::super::{EngineId, Provider};

pub struct NlcProvider;

#[async_trait::async_trait]
impl Provider for NlcProvider {
    fn engine(&self) -> EngineId {
        EngineId::Nlc
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

const ENDPOINT: &str = "https://m.nlc.cn/nlc-api/api/nlc/alephSearch/params";

/// Mirrors the National Library SPA's unusual canonicalizer: object keys are
/// sorted recursively and arrays become objects with numeric string keys.
fn canonicalize(value: &Value, post_method_for_this_level: bool) -> Value {
    match value {
        Value::Object(object) => {
            let mut keys: Vec<_> = object.keys().collect();
            keys.sort_unstable();
            let result = keys
                .into_iter()
                .map(|key| {
                    let child = &object[key];
                    let normalized = match child {
                        Value::Object(_) | Value::Array(_) => canonicalize(child, false),
                        Value::Number(number) if !post_method_for_this_level => {
                            Value::String(number.to_string())
                        }
                        other => other.clone(),
                    };
                    (key.clone(), normalized)
                })
                .collect::<Map<_, _>>();
            Value::Object(result)
        }
        Value::Array(items) => Value::Object(
            items
                .iter()
                .enumerate()
                .map(|(index, item)| (index.to_string(), canonicalize(item, false)))
                .collect(),
        ),
        Value::Number(number) if !post_method_for_this_level => Value::String(number.to_string()),
        Value::Number(number) => Value::Number(number.clone()),
        other => other.clone(),
    }
}

fn signature(body: &Value, sign_time: &str) -> String {
    let canonical = canonicalize(body, true);
    format!("{:x}", md5::compute(format!("{}{sign_time}", canonical))).to_uppercase()
}

pub async fn search(
    client: &reqwest::Client,
    query: &str,
    page: u32,
    page_size: u32,
) -> Result<SearchResponse, ApiError> {
    let body = json!({
        "pageNum": page.to_string(),
        "pageSize": page_size.to_string(),
        "base": "NLC01S",
        "adjacent": "Y",
        "params": [{ "key": query, "code": "WRD" }]
    });
    let sign_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string();
    let response = client
        .post(ENDPOINT)
        .header("sign", signature(&body, &sign_time))
        .header("signTime", sign_time)
        .header("contentType", "application/json")
        .json(&body)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(ApiError::upstream("国家图书馆", response.status()));
    }
    let payload: Value = response.json().await?;
    let data = payload.get("data").unwrap_or(&payload);
    let books = data
        .get("content")
        .and_then(Value::as_array)
        .ok_or(ApiError::InvalidResponse {
            provider: "国家图书馆",
        })?;
    let items = books
        .iter()
        .enumerate()
        .map(|(index, book)| {
            let id = optional_string(book, &["docNumber", "id", "code"])
                .unwrap_or_else(|| format!("nlc-{page}-{index}"));
            let base = optional_string(book, &["base"]).unwrap_or_else(|| "NLC01S".into());
            let doc_number = optional_string(book, &["docNumber"]);
            let detail_url = doc_number.map(|number| {
                format!("https://m.nlc.cn/#/bookDetail?base={base}&docNumber={number}")
            });
            BookResult {
                id,
                title: optional_string(book, &["title"]).unwrap_or_else(|| "未命名书目".into()),
                subtitle: None,
                authors: string_list(book, &["author"]),
                publisher: optional_string(book, &["publishHouse"]),
                published_at: optional_string(book, &["publishYear"]),
                cover_url: optional_string(book, &["cover", "coverUrl"]),
                isbn: optional_string(book, &["isbn"]),
                summary: optional_string(book, &["summary"]),
                rating: None,
                detail_url,
                source: "nlc",
            }
        })
        .collect();
    Ok(SearchResponse {
        items,
        total: data
            .get("totalElements")
            .and_then(Value::as_u64)
            .unwrap_or(books.len() as u64),
        page,
        page_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalization_matches_the_spa() {
        let body = json!({"pageNum":"1","pageSize":"10","base":"NLC01S","adjacent":"Y","params":[{"key":"红楼梦","code":"WRD"}]});
        assert_eq!(
            canonicalize(&body, true).to_string(),
            r#"{"adjacent":"Y","base":"NLC01S","pageNum":"1","pageSize":"10","params":{"0":{"code":"WRD","key":"红楼梦"}}}"#
        );
    }
}
