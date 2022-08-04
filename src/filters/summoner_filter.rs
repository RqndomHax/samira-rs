use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct SummonerFilter {
    pub account_id: Option<String>,
    pub name: Option<String>,
    pub id: Option<String>,
    pub puuid: Option<String>,
}
