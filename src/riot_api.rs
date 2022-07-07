use ureq::serde_json;
use crate::{region::*, platform::*};

#[derive(Debug, PartialEq)]
pub struct RiotApi {
    token: String,
}

impl RiotApi {
    /// Creates a new RiotApi with a token
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
            })
        }
        None
    }
}

fn check_token(token: &str) -> Result<bool, ureq::Error> {
    let request = format!(
        "{server}/lol/status/v4/platform-data",
        server = get_platform_url(&Platform::NA1),
    );
    let response: serde_json::Value = ureq::get(&request)
        .set("X-Riot-Token", token)
        .call()?
        .into_json()?;
    Ok(true)
}
