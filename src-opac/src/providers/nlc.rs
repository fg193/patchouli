use serde_json::{json, Map, Value};

use super::super::{
    error::ApiError,
    models::{optional_string, split_catalog_title, string_list, BookResult, SearchResponse},
};
use super::super::{Provider, ProviderId};
use super::cover_url;

pub struct NlcProvider;

#[async_trait::async_trait]
impl Provider for NlcProvider {
    fn provider(&self) -> ProviderId {
        ProviderId::Nlc
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

fn isbn(value: &Value) -> Option<String> {
    value
        .get("isbn")
        .and_then(Value::as_array)
        .and_then(|values| values.iter().find_map(Value::as_str))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

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
        // pageNum: 页码（字符串）
        // pageSize: 分页大小（字符串）
        // base: 目录库标识，当前使用 NLC01S
        // adjacent: Y=启用相邻匹配，N=关闭相邻匹配
        // params.key: 查询词
        // params.code: WRD=词语检索
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
        return Err(ApiError::http(response.status()));
    }
    let payload: Value = response.json().await?;
    if let Some(code) = payload.get("code").and_then(Value::as_u64) {
        if code != 200 {
            let message = optional_string(&payload, &["msg"]).unwrap_or_default();
            return Err(ApiError::Business { code, message });
        }
    }
    let data = payload.get("data").unwrap_or(&payload);
    let books = data
        .get("content")
        .and_then(Value::as_array)
        .ok_or_else(|| ApiError::invalid_response(&payload))?;
    let items = books
        .iter()
        .filter_map(|book| {
            let id = optional_string(book, &["docNumber"])?;
            let base = optional_string(book, &["base"]).unwrap_or_else(|| "NLC01S".into());
            let detail_url = Some(format!(
                "https://m.nlc.cn/#/bookDetail?base={base}&docNumber={id}"
            ));
            let (title, document_type, subtitles, authors) = split_catalog_title(
                &optional_string(book, &["title"]).unwrap_or_default(),
                optional_string(book, &["type"]),
                string_list(book, &["author"]),
            );
            let isbn = isbn(book);
            Some(BookResult {
                provider_id: ProviderId::Nlc,
                id,
                title,
                subtitles,
                document_type,
                classmark: optional_string(book, &["code"]),
                authors,
                publisher: optional_string(book, &["publishHouse"]),
                publication_date: optional_string(book, &["publishYear"]),
                cover_url: cover_url(isbn.as_deref()),
                isbn,
                summary: optional_string(book, &["summary"]),
                rating: None,
                detail_url,
            })
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
