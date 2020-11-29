use clap::{App, Arg};
use dirs;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::process::exit;
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
    println!("deserialized = {:?}", config);

    let dr = config.default_reviewer;

    let matches = app.get_matches();

    let id = matches.value_of("id").unwrap();
    let l = matches.value_of("link").unwrap();
    let r = matches.value_of("reviewer").unwrap_or("");
    println!("id: {} links:{} r:{}", id, l, r);

    if r.is_empty() && dr.is_empty() {
        println!("You haven't provided any reviewer.");
        exit(1);
    }

    let mut rs = String::new();
    if r.is_empty() {
        rs.push_str(&format!("@{}\n", dr));
    }

    let revs: Vec<&str> = r.split(",").collect();
    if revs.len() > 0 && !revs[0].is_empty() {
        dbg!(1);
        for rev in revs {
            rs.push_str(&format!("@{}\n", rev));
        }
    }

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

fn reviewer() {
    todo!()
}

fn links() {
    todo!()
}

fn create_comment() {
    todo!()
}
