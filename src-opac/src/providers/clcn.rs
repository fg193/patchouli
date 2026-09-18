use serde_json::Value;
use sha1::{Digest, Sha1};

use super::super::{
    error::ApiError,
    models::{optional_string, split_catalog_title, string_list, BookResult, SearchResponse},
};
use super::super::{Provider, ProviderId};
use super::cover_url;

pub struct ClcnProvider;

#[async_trait::async_trait]
impl Provider for ClcnProvider {
    fn provider(&self) -> ProviderId {
        ProviderId::Clcn
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
    // 从 1 开始的翻页偏移量
    let start = (page - 1) * page_size + 1;
    // find_type: 要检索的字段
    //   title=题名，creator=作者，sub=主题，desc=摘要
    //   isbn=ISBN，lsr01=索取号，lsr03=出版社
    // facettype: 资源类型
    //   books=图书，journals=期刊，media=视听资料
    //   rare_books=古籍，dissertations=学位论文，留空表示全部类型
    // cdate: 出版年份筛选，取值来自响应 facets
    // library: 馆藏范围
    //   clcn=首图馆藏，all=全市馆藏
    // loc: 馆藏地筛选，取值来自响应 facets，例如城图.立体书库中文库本^
    // 不做筛选的字段仍需显式传入空值，否则会搜不到结果
    vec![
        ("query", query.to_owned()),
        ("find_type", "title".into()),
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
    let response = client
        .get(ENDPOINT)
        .header("timestamp", &timestamp)
        .header("sign", sign)
        .query(&params)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(ApiError::http(response.status()));
    }
    let payload: Value = response.json().await?;
    let data = payload.get("data").unwrap_or(&payload);
    let rows = data
        .get("rows")
        .or_else(|| data.get("list"))
        .or_else(|| data.get("content"))
        .and_then(Value::as_array)
        .ok_or_else(|| ApiError::invalid_response(&payload))?;
    let items = rows
        .iter()
        .filter_map(|book| {
            let id = optional_string(book, &["ilsapiid"])?;
            let isbn = optional_string(book, &["identifier"])
                .map(|isbn| isbn.trim_start_matches("ISBN").trim().to_owned());
            let (title, document_type, subtitles, authors) = split_catalog_title(
                &optional_string(book, &["title"]).unwrap_or_default(),
                optional_string(book, &["coverage"]),
                string_list(book, &["creator"]),
            );
            Some(BookResult {
                provider_id: ProviderId::Clcn,
                id,
                title,
                subtitles,
                document_type,
                classmark: None,
                authors,
                publisher: optional_string(book, &["publisher"]),
                publication_date: optional_string(book, &["creationdate"]),
                cover_url: cover_url(isbn.as_deref()),
                isbn,
                summary: optional_string(book, &["description"]),
                rating: None,
                detail_url: None,
            })
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
        const BOOK_TITLE: &str = "Designing Data-Intensive Applications";
        assert_eq!(
            search_params(BOOK_TITLE, 1, 9),
            vec![
                ("query", BOOK_TITLE.into()),
                ("find_type", "title".into()),
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
