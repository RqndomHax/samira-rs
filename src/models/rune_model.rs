use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct RuneData {
    pub id: i32,
    pub key: String,
    pub icon: String,
    pub name: String,
    #[serde(alias = "shortDesc")]
    pub short_desc: String,
    #[serde(alias = "longDesc")]
    pub long_desc: String,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct RuneSlot {
    pub runes: Vec<RuneData>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct Rune {
    pub id: i32,
    pub key: String,
    pub icon: String,
    pub name: String,
    pub slots: Vec<RuneSlot>,
}
