use ureq::serde_json::{self, Value};

use crate::models::champion_model::*;
use crate::models::rune_model::*;

const SERVER: &str = "https://ddragon.leagueoflegends.com";

#[derive(Debug, PartialEq)]
pub struct UtilsApi {
    pub version: String,
    pub language: String,
}

impl Default for UtilsApi {
    fn default() -> UtilsApi {
        UtilsApi {
            version: "12.12.1".to_string(),
            language: "en_US".to_string(),
        }
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
            Some(UtilsApi {
                version: version.unwrap(),
                language: language.to_owned(),
            })
        } else {
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
        if (language_result.is_ok() && language_result.unwrap() == true)
            && (version_result.is_ok() && version_result.unwrap() == true)
        {
            return Some(UtilsApi {
                version: version.to_owned(),
                language: language.to_owned(),
            });
        }
        None
    }

    /// Retrieve all current champions
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use samira::{models::champion_model::*, utils_api::*};
    ///
    /// let api = UtilsApi::new("12.12.1", "fr_FR").unwrap_or_default();
    /// let champions = api.get_all_champions();
    /// assert_eq!(champions.iter().find(|&c| c.name == "Samira").is_some(), true);
    /// assert_eq!(champions.iter().find(|&c| c.name == "Akali").is_some(), true);
    /// assert_eq!(champions.iter().find(|&c| c.name == "RqndomChampion").is_some(), false);
    /// ```
    pub fn get_all_champions(&self) -> Vec<Champion> {
        let champions = get_all_champions(&self.version, &self.language);
        if champions.is_ok() {
            return champions.unwrap();
        }
        Vec::new()
    }

    /// Retrieve a champion from its name
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use samira::{models::champion_model::*, utils_api::*};
    ///
    /// let api = UtilsApi::latest("en_US").unwrap_or_default();
    /// assert_eq!("Samira", api.get_champion("Samira".to_owned()).unwrap().name);
    pub fn get_champion(&self, name: String) -> Option<Champion> {
        let champion = get_champion(&self.version, &self.language, name);
        if champion.is_ok() {
            return Some(champion.unwrap());
        }
        None
    }

    /// Retrieve a rune from its name
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use samira::{models::rune_model::*, utils_api::*};
    ///
    /// let api = UtilsApi::latest("en_US").unwrap_or_default();
    /// assert_eq!("Domination", api.get_rune("Domination".to_owned()).unwrap().name);
    /// assert_eq!("Inspiration", api.get_rune("Inspiration".to_owned()).unwrap().name);
    pub fn get_rune(&self, name: String) -> Option<Rune> {
        let rune = get_rune(&self.version, &self.language, name);
        if rune.is_ok() {
            return Some(rune.unwrap());
        }
        None
    }

    /// Retrieve all current runes
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use samira::{models::rune_model::*, utils_api::*};
    ///
    /// let api = UtilsApi::new("12.12.1", "fr_FR").unwrap_or_default();
    /// let runes = api.get_all_runes();
    /// assert_eq!(runes.iter().find(|&c| c.name == "Domination").is_some(), true);
    /// assert_eq!(runes.iter().find(|&c| c.name == "Inspiration").is_some(), true);
    /// assert_eq!(runes.iter().find(|&c| c.id == 8400).is_some(), true);
    /// assert_eq!(runes.iter().find(|&c| c.name == "RqndomRune").is_some(), false);
    /// ```
    pub fn get_all_runes(&self) -> Vec<Rune> {
        let runes = get_all_runes(&self.version, &self.language);
        if runes.is_ok() {
            return runes.unwrap();
        }
        Vec::new()
    }
}

fn get_all_champions(version: &String, language: &String) -> Result<Vec<Champion>, ureq::Error> {
    let mut champions: Vec<Champion> = Vec::new();
    let request = format!(
        "{SERVER}/cdn/{version}/data/{language}/championFull.json",
        SERVER = SERVER,
        version = version,
        language = language,
    );
    let response: serde_json::Value = ureq::get(&request).call()?.into_json()?;

    let champ = response
        .as_object()
        .expect("not an object")
        .get("data")
        .expect("no data found")
        .as_object()
        .expect("no champions found");

    for val in champ.values() {
        champions.push(serde_json::from_value(val.clone()).unwrap());
    }

    Ok(champions)
}

fn get_champion(
    version: &String,
    language: &String,
    name: String,
) -> Result<Champion, ureq::Error> {
    let request = format!(
        "{SERVER}/cdn/{version}/data/{language}/championFull.json",
        SERVER = SERVER,
        version = version,
        language = language,
    );
    let response: serde_json::Value = ureq::get(&request).call()?.into_json()?;

    let champ = response
        .as_object()
        .expect("not an object")
        .get("data")
        .expect("no data found")
        .as_object()
        .expect("no champions found")
        .get(&name)
        .expect("champion not found");

    Ok(serde_json::from_value(champ.clone()).unwrap())
}

fn get_all_runes(version: &String, language: &String) -> Result<Vec<Rune>, ureq::Error> {
    let mut runes = Vec::new();
    let request = format!(
        "{SERVER}/cdn/{version}/data/{language}/runesReforged.json",
        SERVER = SERVER,
        version = version,
        language = language,
    );
    let response: serde_json::Value = ureq::get(&request).call()?.into_json()?;

    let rune = response.as_array().expect("not an array");

    for val in rune {
        runes.push(serde_json::from_value(val.clone()).unwrap());
    }

    Ok(runes)
}

fn get_rune(version: &String, language: &String, name: String) -> Result<Rune, ureq::Error> {
    let request = format!(
        "{SERVER}/cdn/{version}/data/{language}/runesReforged.json",
        SERVER = SERVER,
        version = version,
        language = language,
    );
    let response: serde_json::Value = ureq::get(&request).call()?.into_json()?;

    let rune = response.as_array().expect("not an array");
    let mut target = None;

    for val in rune {
        if val
            .as_object()
            .expect("not an object")
            .get("name")
            .expect("name not found")
            .as_str()
            .expect("not a string")
            == name
        {
            target = Some(val);
        }
    }

    Ok(serde_json::from_value(target.unwrap().clone()).unwrap())
}

fn get_latest_version() -> Result<String, ureq::Error> {
    let request = format!("{SERVER}/api/versions.json", SERVER = SERVER,);
    let response: serde_json::Value = ureq::get(&request).call()?.into_json()?;
    Ok(response
        .as_array()
        .expect("not an array")
        .get(0)
        .expect("no latest version")
        .as_str()
        .expect("not a string")
        .to_string())
}

fn is_version_available(version: String) -> Result<bool, ureq::Error> {
    let request = format!("{SERVER}/api/versions.json", SERVER = SERVER,);
    let response: serde_json::Value = ureq::get(&request).call()?.into_json()?;
    Ok(response
        .as_array()
        .expect("not an array")
        .contains(&Value::String(version.to_string())))
}

fn is_language_available(language: String) -> Result<bool, ureq::Error> {
    let request = format!("{SERVER}/cdn/languages.json", SERVER = SERVER,);
    let response: serde_json::Value = ureq::get(&request).call()?.into_json()?;
    Ok(response
        .as_array()
        .expect("not an array")
        .contains(&Value::String(language.to_string())))
}
