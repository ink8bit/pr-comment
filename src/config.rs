use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, path::PathBuf};

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

pub fn path(home_path: PathBuf, config_file: &str) -> String {
    let home_str_path = home_path.to_str().unwrap();
    let config_path = format!("{}/{}", home_str_path, config_file);

    config_path
}

pub fn parse(config_file: String) -> Result<Config, Box<dyn Error>> {
    let config_data = fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&config_data)?;

    Ok(config)
}
