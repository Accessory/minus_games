use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, strum::Display, ToSchema,
)]
#[serde(rename_all = "lowercase")]
#[schema(examples(true, false), as = bool)]
pub enum Boolean {
    True,
    False,
}

impl Boolean {
    pub fn create_json_string(value: bool) -> String {
        if value {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}
