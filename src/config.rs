//! Configuration options

use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs};

/// Config file name, format - `JSON`.
pub const FILE: &str = ".commentrc";

#[derive(Deserialize, Debug)]
pub struct LinkInfo {
    #[serde(rename(deserialize = "repoName"))]
    pub repo_name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename(deserialize = "defaultReviewer"))]
    pub default_reviewer: String,
    pub links: HashMap<String, LinkInfo>,
}

impl Config {
    /// Creates Config struct from config file contents.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config_file = Self::path();
        let config = Self::parse(config_file);
        match config {
            Ok(v) => Ok(Self {
                default_reviewer: v.default_reviewer,
                links: v.links,
            }),
            Err(e) => Err(e),
        }
    }

    /// Resolve path to config file `$HOME/.commentrc`.
    fn path() -> String {
        let home_path = dirs::home_dir();
        match home_path {
            Some(p) => {
                let home_str_path = p.to_str().unwrap();
                let config_path = format!("{}/{}", home_str_path, FILE);
                config_path
            }
            None => panic!("Could not get $HOME dir path"),
        }
    }

    /// Parse config file.
    fn parse(config_file: String) -> Result<Self, Box<dyn Error>> {
        let config_data = fs::read_to_string(config_file)?;
        let config = serde_json::from_str(&config_data)?;
        Ok(config)
    }
}
