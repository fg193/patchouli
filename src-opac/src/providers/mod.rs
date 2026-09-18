mod clcn;
mod douban;
mod nlc;

use super::Provider;

pub(crate) const COVER_ENDPOINT: &str = "https://business-center-ops.clcn.net.cn/ccbd/getPic?isbn=";

pub(crate) fn cover_url(isbn: Option<&str>) -> Option<String> {
    isbn.map(|isbn| format!("{COVER_ENDPOINT}{isbn}"))
}

#[cfg(test)]
mod tests {
    use super::cover_url;

    #[test]
    fn builds_cover_url_only_when_isbn_exists() {
        assert_eq!(
            cover_url(Some("9787541096372")),
            Some("https://business-center-ops.clcn.net.cn/ccbd/getPic?isbn=9787541096372".into())
        );
        assert_eq!(cover_url(None), None);
    }
}

pub fn all() -> Vec<Box<dyn Provider>> {
    vec![
        Box::new(clcn::ClcnProvider),
        Box::new(douban::DoubanProvider),
        Box::new(nlc::NlcProvider),
    ]
}
