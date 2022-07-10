use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ChampionInfo {
    #[serde(alias = "maxNewPlayerLevel")]
    pub max_new_player_level: i32,
    #[serde(alias = "freeChampionIdsForNewPlayers")]
    pub free_champions_ids_for_new_players: Vec<i32>,
    #[serde(alias = "freeChampionIds")]
    pub free_champion_ids: Vec<i32>,
}
