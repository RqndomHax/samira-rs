use crate::{models::summoner_model::*, platform::*, region::*};
use ureq::serde_json;

#[derive(Debug, PartialEq)]
pub struct RiotApi {
    token: String,
}

impl RiotApi {
    /// Creates a new RiotApi with a token.
    /// It checks if the token is valid by retrieving the League of Legends NA1 region status.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::process::exit;
    /// use samira::riot_api::*;
    ///
    /// let api = RiotApi::new("TOKEN_HERE");
    /// if api.is_none() {
    ///     // We exit the program because the token is invalid
    ///     exit(0);
    /// }
    /// let api = api.unwrap();
    /// // We can now use the api methods.
    /// ```
    pub fn new(token: &str) -> Option<RiotApi> {
        let result = check_token(token);
        if result.is_ok() && result.unwrap() == true {
            return Some(RiotApi {
                token: token.to_string(),
            });
        }
        None
    }

    /// Creates a new RiotApi with a token.
    /// It doesn't check if the token is valid.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use samira::riot_api::*;
    ///
    /// let api = RiotApi::new_unchecked("TOKEN_HERE");
    /// ```
    pub fn new_unchecked(token: &str) -> RiotApi {
        return RiotApi {
            token: token.to_string(),
        };
    }

    /// Retrieve a summoner by its puuid.
    /// If the summoner does not exist it returns None.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use samira::{riot_api::*, platform::*};
    ///
    /// let api = RiotApi::new("RGAPI-db744840-2107-4a1c-931d-3a1e95b7ad8b").unwrap();
    /// let puuid = "Y22N0dvmtG6NsF5GTpPJ4yhxI2t3zMvP5solMwWSqj1Ld-YAijBqMG5bDP9xYZ9EgVkyxiyifsMC_Q";
    /// let summoner = api.get_summoner_by_puuid(&Platform::EUW1, &puuid);
    /// assert_eq!(summoner.unwrap().puuid, puuid);
    /// ```
    pub fn get_summoner_by_puuid(&self, platform: &Platform, puuid: &str) -> Option<Summoner> {
        let summoner_result = get_summoner_by_puuid(&self.token, platform, puuid);
        if summoner_result.is_ok() {
            return Some(summoner_result.unwrap());
        }
        None
    }
}

fn get_summoner_by_puuid(
    token: &str,
    platform: &Platform,
    puuid: &str,
) -> Result<Summoner, ureq::Error> {
    let request = format!(
        "{server}/lol/summoner/v4/summoners/by-puuid/{puuid}",
        server = get_platform_url(platform),
        puuid = puuid
    );
    let response: serde_json::Value = ureq::get(&request)
        .set("X-Riot-Token", token)
        .call()?
        .into_json()?;

    Ok(serde_json::from_value(response).unwrap())
}

fn check_token(token: &str) -> Result<bool, ureq::Error> {
    let request = format!(
        "{server}/lol/status/v4/platform-data",
        server = get_platform_url(&Platform::NA1),
    );
    ureq::get(&request)
        .set("X-Riot-Token", token)
        .call()?
        .into_json()?;
    Ok(true)
}
