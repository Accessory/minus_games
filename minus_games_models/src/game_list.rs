use crate::game_infos::{GameInfos, MinimalGameInfos};
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
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct GamesWithGameInfos {
    pub name: String,
    pub date: DateTime<Utc>,
    pub header: bool,
    pub game_infos: GameInfos,
}

impl GamesWithGameInfos {
    pub fn new(name: String, date: DateTime<Utc>, header: bool, game_infos: GameInfos) -> Self {
        Self {
            name,
            date,
            header,
            game_infos,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct GamesWithMinimalGameInfos {
    pub name: String,
    pub date: DateTime<Utc>,
    pub header: bool,
    pub minimal_game_infos: MinimalGameInfos,
}

impl GamesWithMinimalGameInfos {
    pub fn new(
        name: String,
        date: DateTime<Utc>,
        header: bool,
        minimal_game_infos: MinimalGameInfos,
    ) -> Self {
        Self {
            name,
            date,
            header,
            minimal_game_infos,
        }
    }
}
