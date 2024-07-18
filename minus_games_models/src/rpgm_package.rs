use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RPGMPackage {
    pub name: String,
    pub main: String,
    #[serde(rename = "js-flags")]
    pub js_flags: String,
    #[serde(rename = "chromium-args")]
    pub chromium_args: String,
    pub window: Window,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Window {
    pub title: String,
    pub toolbar: bool,
    pub width: i64,
    pub height: i64,
    pub icon: String,
}

impl Display for RPGMPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).expect("Failed to serialize to json")
        )
    }
}
