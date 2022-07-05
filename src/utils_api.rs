use ureq::serde_json::{self, Value};

use crate::models::champion_model::*;

const SERVER: &str = "https://ddragon.leagueoflegends.com";

#[derive(Debug, PartialEq)]
pub struct UtilsApi {
    pub version: String,
    pub language: String,
}

impl Default for UtilsApi {
    fn default() -> UtilsApi {
        UtilsApi {version: "12.12.1".to_string(), language: "en_US".to_string()}
    }
}

impl UtilsApi {

    /// Creates a new UtilsApi using the latest available version and custom language.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// (current latest version is 12.12.1 (07/05/2022))
    /// ```
    /// use samira::utils_api::*;
    /// 
    /// let api = UtilsApi::latest("en_US").unwrap_or_default();
    /// assert_eq!(api, UtilsApi{version: "12.12.1".to_owned(), language: "en_US".to_owned()});
    /// ```
    pub fn latest(language: &str) -> Option<UtilsApi> {
        let language_result = is_language_available(language.to_owned());
        let version = get_latest_version();
        if version.is_ok() && (language_result.is_ok() && language_result.unwrap() == true) {
            Some(UtilsApi {version: version.unwrap(), language: language.to_owned()})
        }
        else {
            None
        }
    }

    /// Creates a new UtilsApi using a custom version and custom language.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use samira::utils_api::*;
    /// 
    /// let api = UtilsApi::new("12.12.1", "fr_FR").unwrap_or_default();
    /// assert_eq!(api, UtilsApi{version: "12.12.1".to_owned(), language: "fr_FR".to_owned()});
    /// ```
    pub fn new(version: &str, language: &str) -> Option<UtilsApi> {
        let version_result = is_version_available(version.to_owned());
        let language_result = is_language_available(language.to_owned());
        if (language_result.is_ok() && language_result.unwrap() == true) && (version_result.is_ok() && version_result.unwrap() == true) {
            Some(UtilsApi {version: version.to_owned(), language: language.to_owned()})
        }
        else {
            None
        }
    }

    /// get Champion structure from its name
    pub fn get_champion(name: String) -> Option<Champion> {
        None
    }

}

fn get_latest_version() -> Result<String, ureq::Error> {
    let request = format!(
        "{SERVER}/api/versions.json",
        SERVER = SERVER,
    );
    let response: serde_json::Value = ureq::get(&request)
        .call()?
        .into_json()?;
    Ok(response.as_array().expect("not an array").get(0).expect("no latest version").as_str().expect("not a string").to_string())
}

fn is_version_available(version: String) -> Result<bool, ureq::Error> {
    let request = format!(
        "{SERVER}/api/versions.json",
        SERVER = SERVER,
    );
    let response: serde_json::Value = ureq::get(&request)
        .call()?
        .into_json()?;
    Ok(response.as_array().expect("not an array").contains(&Value::String(version.to_string())))
}

fn is_language_available(language: String) -> Result<bool, ureq::Error> {
    let request = format!(
        "{SERVER}/cdn/languages.json",
        SERVER = SERVER,
    );
    let response: serde_json::Value = ureq::get(&request)
        .call()?
        .into_json()?;
    Ok(response.as_array().expect("not an array").contains(&Value::String(language.to_string())))
}
