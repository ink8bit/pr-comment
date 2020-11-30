use dirs;
use serde::Deserialize;

use std::{collections::HashMap, error::Error, fs};

pub const FILE: &str = ".commentrc";

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename(deserialize = "defaultReviewer"))]
    pub default_reviewer: String,
    pub links: HashMap<String, LinkInfo>,
}
#[derive(Deserialize, Debug)]
pub struct LinkInfo {
    pub description: String,
    pub url: String,
}

pub fn path(config_file: &str) -> String {
    let home_path = dirs::home_dir();
    match home_path {
        Some(p) => {
            let home_str_path = p.to_str().unwrap();
            let config_path = format!("{}/{}", home_str_path, config_file);
            config_path
        }
        None => panic!("can't get $HOME dir path"),
    }
}

pub fn parse(config_file: String) -> Result<Config, Box<dyn Error>> {
    let config_data = fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&config_data)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path() {
        assert_eq!(
            path(FILE),
            format!("{}/{}", dirs::home_dir().unwrap().to_str().unwrap(), FILE)
        );
    }
}
