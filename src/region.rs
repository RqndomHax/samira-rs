const PROTOCOL: &str = "https";

pub enum Region {
    AMERICAS,
    ASIA,
    EUROPE,
    SEA,
}

pub fn get_region_url(region: &Region) -> String {
    format!("{protocol}://{region}.api.riotgames.com", protocol = PROTOCOL, region = match region {
        Region::AMERICAS => "americas",
        Region::ASIA => "asia",
        Region::EUROPE => "europe",
        Region::SEA => "sea",
    })
}
