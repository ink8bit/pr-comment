use clap::{App, Arg};
use dirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

const CONFIG_FILE: &str = ".commentrc.json";

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(rename(deserialize = "defaultReviewer"))]
    default_reviewer: String,
    #[serde(rename(deserialize = "templatePath"))]
    template_path: String,
    links: HashMap<String, LinkInfo>,
}
#[derive(Deserialize, Debug)]
struct LinkInfo {
    abbreviation: String,
    description: String,
    url: String,
}

fn main() {
    let app = App::new("comment")
        .version("0.0.1")
        .about("Creates PR comment")
        .author("ink8bit")
        .arg(
            Arg::new("id")
                .short('i')
                .long("id")
                .value_name("int")
                .about("Sets Task ID value")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("link")
                .short('l')
                .long("link")
                .value_name("string")
                .about("Sets PR links values, use comma for multiple values")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("reviewer")
                .short('r')
                .long("reviewer")
                .value_name("string")
                .about("Sets a reviewer or reviewers, use comma for multiple values")
                .takes_value(true),
        );

    let matches = app.get_matches();
    println!("{:?}", matches);

    let home_path = dirs::home_dir().expect("can't get $HOME dir path");
    let home_str_path = home_path.to_str().unwrap();
    let config_path = format!("{}/{}", home_str_path, CONFIG_FILE);
    println!("{:?}", config_path);

    let config_data = fs::read_to_string(config_path).expect("Unable to read config file");
    println!("{}", config_data);

    let deserialized: Config = serde_json::from_str(&config_data).unwrap();
    println!("deserialized = {:?}", deserialized);
}
