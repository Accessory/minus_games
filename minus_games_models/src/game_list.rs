use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct GamesWithDate {
    pub name: String,
    pub date: DateTime<Utc>,
}

impl GamesWithDate {
    pub fn new(name: String, date: DateTime<Utc>) -> Self {
        Self { name, date }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct GamesWithInfos {
    pub name: String,
    pub date: DateTime<Utc>,
    pub header: bool,
}

impl GamesWithInfos {
    pub fn new(name: String, date: DateTime<Utc>, header: bool) -> Self {
        Self { name, date, header }
    }
}
