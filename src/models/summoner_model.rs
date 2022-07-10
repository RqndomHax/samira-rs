use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Summoner {
    #[serde(alias = "accountId")]
    pub account_id: String,
    #[serde(alias = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(alias = "revisionDate")]
    pub revision_date: i64,
    pub name: String,
    pub id: String,
    pub puuid: String,
    #[serde(alias = "summonerLevel")]
    pub summoner_level: i64,
}
