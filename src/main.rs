use clap::{App, Arg};
use dirs;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

const CONFIG_FILE: &str = ".commentrc";

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(rename(deserialize = "defaultReviewer"))]
    default_reviewer: String,
    links: HashMap<String, LinkInfo>,
}
#[derive(Deserialize, Debug)]
struct LinkInfo {
    description: String,
    url: String,
}

struct Comment {
    id: u16,
    reviewers: String,
    links: String,
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

    let home_path = dirs::home_dir().expect("can't get $HOME dir path");
    let config_file = config_path(home_path, CONFIG_FILE);
    let config = parse_config(config_file).expect("can't parse config file");

    let matches = app.get_matches();
    let id = matches.value_of("id").unwrap();
    let l = matches.value_of("link").unwrap();
    let r = matches.value_of("reviewer").unwrap_or("");
    let dr = config.default_reviewer;
    let rs = reviewers(r, dr).expect("can't get a list of reviewers.");

    let links: Vec<&str> = l.split(",").collect();
    let mut s = String::new();

    if links.len() > 0 {
        for link in links {
            if config.links.contains_key(link) {
                let val = config.links.get(link).unwrap();
                s.push_str(&format!("- [{}]({})\n", val.description, val.url));
            }
        }
    }

    let template = format!(
        "
**PR**
`feature/{}`

**LINKS**
{}

**REVIEW**
{}

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
",
        id, s, rs,
    );

    println!("{}", template);
}

fn config_path(home_path: PathBuf, config_file: &str) -> String {
    let home_str_path = home_path.to_str().unwrap();
    let config_path = format!("{}/{}", home_str_path, config_file);
    config_path
}

fn parse_config(config_file: String) -> Result<Config, Box<dyn Error>> {
    let config_data = fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}

fn reviewers(r_flag_value: &str, default_reviewer: String) -> Result<String, Box<dyn Error>> {
    if r_flag_value.is_empty() && default_reviewer.is_empty() {
        panic!("you haven't provided any reviewer.");
    }

    let mut rs = String::new();
    if r_flag_value.is_empty() {
        rs.push_str(&format!("@{}\n", default_reviewer));
    }

    let revs: Vec<&str> = r_flag_value.split(",").collect();
    if revs.len() > 0 && !revs[0].is_empty() {
        for rev in revs {
            rs.push_str(&format!("@{}\n", rev));
        }
    }

    Ok(rs)
}

fn links() {
    todo!()
}

fn create_comment() {
    todo!()
}
