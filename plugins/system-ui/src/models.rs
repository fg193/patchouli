use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusBarStyle {
    pub background_color: String,
    pub dark_icons: bool,
}
