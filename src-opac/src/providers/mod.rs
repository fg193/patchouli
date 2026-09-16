mod clcn;
mod douban;
mod nlc;

use super::Provider;

pub fn all() -> Vec<Box<dyn Provider>> {
    vec![
        Box::new(clcn::ClcnProvider),
        Box::new(douban::DoubanProvider),
        Box::new(nlc::NlcProvider),
    ]
}
