use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Passive {
    pub name: String,
    pub description: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct LevelTip {
    pub label: Vec<String>,
    pub effect: Vec<String>,
}

#[derive(Deserialize, Serialize, Default, Debug, PartialEq)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub leveltip: LevelTip,
    pub maxrank: i32,
    pub cooldown: Vec<f64>,
    #[serde(alias = "cooldownBurn")]
    pub cooldown_burn: String,
    pub cost: Vec<f64>,
    #[serde(alias = "costBurn")]
    pub cost_burn: String,
    pub effect: Vec<Option<Vec<f64>>>,
    #[serde(alias = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    #[serde(alias = "costType")]
    pub cost_type: String,
    pub maxammo: String,
    pub range: Vec<i64>,
    #[serde(alias = "rangeBurn")]
    pub range_burn: String,
    pub image: Image,
    pub resource: String,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Stats {
    pub hp: f64,
    pub hpperlevel: f64,
    pub mp: f64,
    pub mpperlevel: f64,
    pub movespeed: f64,
    pub armor: f64,
    pub armorperlevel: f64,
    pub spellblock: f64,
    pub spellblockperlevel: f64,
    pub attackrange: f64,
    pub hpregen: f64,
    pub hpregenperlevel: f64,
    pub mpregen: f64,
    pub mpregenperlevel: f64,
    pub crit: f64,
    pub critperlevel: f64,
    pub attackdamage: f64,
    pub attackdamageperlevel: f64,
    pub attackspeedperlevel: f64,
    pub attackspeed: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Info {
    pub attack: i32,
    pub defense: i32,
    pub magic: i32,
    pub difficulty: i32,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Skin {
    pub id: String,
    pub num: i32,
    pub name: String,
    pub chromas: bool,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct Champion {
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub image: Image,
    pub skins: Vec<Skin>,
    pub lore: String,
    pub blurb: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub tags: Vec<String>,
    pub partype: String,
    pub info: Info,
    pub stats: Stats,
    pub spells: Vec<Spell>,
    pub passive: Passive,
}
