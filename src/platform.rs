const PROTOCOL: &str = "https";

pub enum Platform {
    BR1,
    EUN1,
    EUW1,
    JP1,
    KR,
    LA1,
    LA2,
    NA1,
    OC1,
    TR1,
    RU
}

pub fn get_platform_url(platform: &Platform) -> String {
    format!("{protocol}://{platform}.api.riotgames.com", protocol = PROTOCOL, platform = match platform {
        Platform::BR1 => "br1",
        Platform::EUN1 => "eun1",
        Platform::EUW1 => "euw1",
        Platform::JP1 => "jp1",
        Platform::KR => "kr",
        Platform::LA1 => "la1",
        Platform::LA2 => "la2",
        Platform::NA1 => "na1",
        Platform::OC1 => "oc1",
        Platform::TR1 => "tr1",
        Platform::RU => "ru",
    })
}
