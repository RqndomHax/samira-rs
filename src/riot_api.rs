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
    /// use std::env;
    /// use std::process::exit;
    /// use samira::riot_api::*;
    ///
    /// let token = env::var("RIOT_API");
    /// if token.is_err() {
    ///     // We exit the program because we couldn't find the token
    ///     exit(0);
    /// }
    /// let token = token.unwrap().to_string();
    /// let api = RiotApi::new(&token);
    /// assert_eq!(api.is_some(), true);
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
    /// use std::env;
    /// use std::process::exit;
    ///
    /// let token = env::var("RIOT_API");
    /// if token.is_err() {
    ///     // We exit the program because we couldn't find the token
    ///     exit(0);
    /// }
    /// let token = token.unwrap().to_string();
    /// use samira::{riot_api::*, platform::*};
    ///
    /// let api = RiotApi::new(&token).unwrap();
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

    /// Retrieve a summoner by its name.
    /// If the summoner does not exist it returns None.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::env;
    /// use std::process::exit;
    ///
    /// let token = env::var("RIOT_API");
    /// if token.is_err() {
    ///     // We exit the program because we couldn't find the token
    ///     exit(0);
    /// }
    /// let token = token.unwrap().to_string();
    /// use samira::{riot_api::*, platform::*};
    ///
    /// let api = RiotApi::new(&token).unwrap();
    /// let name = "RqndomHax";
    /// let summoner = api.get_summoner_by_name(&Platform::EUW1, &name);
    /// assert_eq!(summoner.unwrap().name, name);
    /// ```
    pub fn get_summoner_by_name(
        &self,
        platform: &Platform,
        summoner_name: &str,
    ) -> Option<Summoner> {
        let summoner_result = get_summoner_by_name(&self.token, platform, summoner_name);
        if summoner_result.is_ok() {
            return Some(summoner_result.unwrap());
        }
        None
    }
}

fn get_summoner_by_name(
    token: &str,
    platform: &Platform,
    summoner_name: &str,
) -> Result<Summoner, ureq::Error> {
    let request = format!(
        "{server}/lol/summoner/v4/summoners/by-name/{summoner_name}",
        server = get_platform_url(platform),
        summoner_name = summoner_name
    );
    let response: serde_json::Value = ureq::get(&request)
        .set("X-Riot-Token", token)
        .call()?
        .into_json()?;

    Ok(serde_json::from_value(response).unwrap())
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
