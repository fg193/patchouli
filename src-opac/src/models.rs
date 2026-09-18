use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub provider: ProviderId,
    pub query: String,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderId {
    // douban: 豆瓣图书
    Douban,
    // clcn: 首都图书馆
    Clcn,
    // nlc: 中国国家图书馆
    Nlc,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    pub items: Vec<BookResult>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResult {
    // providerId: 来源系统
    pub provider_id: ProviderId,
    // id: 在来源系统内的主键 ID
    pub id: String,
    // title: 题名
    pub title: String,
    // subtitles: 副题名列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subtitles: Vec<String>,
    // documentType: 文献类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    // classmark: 分类号，不包括同一个分类下的书次号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classmark: Option<String>,
    // authors: 作者列表，对应 creator、author 或 translator
    pub authors: Vec<String>,
    // publisher: 出版社
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    // publicationDate: 出版日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    // coverUrl: 封面图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    // isbn: ISBN，用作跨图书馆联表查询的外键
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    // summary: 内容摘要
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    // rating: 评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f64>,
    // detailUrl: 详情页地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_url: Option<String>,
}

pub fn optional_string(value: &serde_json::Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        let value = value.get(*key)?;
        let text = match value {
            serde_json::Value::String(text) => text.trim().to_owned(),
            serde_json::Value::Number(number) => number.to_string(),
            _ => return None,
        };
        (!text.is_empty()).then_some(text)
    })
}

pub fn string_list(value: &serde_json::Value, keys: &[&str]) -> Vec<String> {
    keys.iter()
        .filter_map(|key| value.get(*key))
        .flat_map(|value| match value {
            serde_json::Value::Array(values) => values
                .iter()
                .filter_map(|item| item.as_str().map(str::trim))
                .filter(|item| !item.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            serde_json::Value::String(text) => text
                .split(&[',', '，', ';', '/'][..])
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect()
}

pub fn split_catalog_title(
    text: &str,
    document_type: Option<String>,
    fallback_authors: Vec<String>,
) -> (String, Option<String>, Vec<String>, Vec<String>) {
    let text = text.trim();
    let (title_part, rest) = text.split_once('/').unwrap_or((text, ""));
    let title_without_metadata = title_part
        .split_once('[')
        .map(|(title, _)| title)
        .unwrap_or(title_part);
    let title = title_without_metadata
        .split_once('=')
        .map(|(title, _)| title)
        .unwrap_or(title_without_metadata)
        .trim();
    let subtitle_source = title_part
        .split_once(']')
        .map(|(_, subtitle)| subtitle)
        .or_else(|| title_part.split_once('=').map(|(_, subtitle)| subtitle))
        .unwrap_or("");
    let subtitles = subtitle_source
        .trim()
        .trim_start_matches(':')
        .trim()
        .split('=')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect();
    let authors = if rest.trim().is_empty() {
        fallback_authors
    } else {
        vec![rest.trim().trim_end_matches(';').trim().to_owned()]
    };
    let document_type = document_type.filter(|value| !value.trim().is_empty());
    (title.to_owned(), document_type, subtitles, authors)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{split_catalog_title, string_list};

    #[test]
    fn combines_all_string_list_fields() {
        let value = json!({
            "author": ["作者一", "作者二"],
            "translator": ["译者一"],
            "editor": "编辑一，编辑二"
        });

        assert_eq!(
            string_list(&value, &["author", "translator", "editor"]),
            vec!["作者一", "作者二", "译者一", "编辑一", "编辑二"]
        );
    }

    #[test]
    fn splits_catalog_titles() {
        let cases = [
            (
                "他们不知道做什么 [专著] = He eivät tiedä mitä tekevät/ (芬)尤西·瓦尔托宁(Jussi Valtonen)著 ; 倪晓京译",
                ("他们不知道做什么", vec!["专著"], vec!["He eivät tiedä mitä tekevät"], vec!["(芬)尤西·瓦尔托宁(Jussi Valtonen)著 ; 倪晓京译"]),
            ),
            (
                "北京的城墙和城门 = The walls and Gates of Peking / (瑞典)喜龙仁(Osvald Sirén)著 ; 林稚晖译",
                ("北京的城墙和城门", vec!["专著"], vec!["The walls and Gates of Peking"], vec!["(瑞典)喜龙仁(Osvald Sirén)著 ; 林稚晖译"]),
            ),
            (
                "奢侈品战争 [专著] : 品牌金字塔与行业巨鳄 = Les guerres du luxe/ (法)史蒂芬·马尚著 ; 潘娥译",
                ("奢侈品战争", vec!["专著"], vec!["品牌金字塔与行业巨鳄", "Les guerres du luxe"], vec!["(法)史蒂芬·马尚著 ; 潘娥译"]),
            ),
            (
                "20世纪20-30年代中国革命者在俄罗斯 / 《20世纪20-30年代中国革命者在俄罗斯》编委会[著]",
                ("20世纪20-30年代中国革命者在俄罗斯", vec!["专著"], vec![], vec!["《20世纪20-30年代中国革命者在俄罗斯》编委会[著]"]),
            ),
        ];

        for (text, expected) in cases {
            let (title, actual_document_type, subtitles, authors) =
                split_catalog_title(text, Some("专著".to_owned()), vec![]);
            assert_eq!(
                (title, actual_document_type, subtitles, authors),
                (
                    expected.0.to_owned(),
                    expected.1.first().map(|value| (*value).to_owned()),
                    expected.2.into_iter().map(str::to_owned).collect(),
                    expected.3.into_iter().map(str::to_owned).collect(),
                )
            );
        }
    }
}
