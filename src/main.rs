use std::{collections::HashMap, error::Error, fs, path::PathBuf};

use clap::{App, Arg};
use dirs;
use serde::Deserialize;

mod comment;

use comment::{create, Comment};

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
        .about("comment is a CLI app which creates a formatted comment for your pull requests.")
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
    let rs = reviewers(r, dr).expect("can't create a list of reviewers.");
    let ls = links(l, config.links);

    let c = create(Comment {
        id: id.to_string(),
        links: ls,
        reviewers: rs,
    });

    println!("{}", c);
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

fn links(l_flag_value: &str, config_links: HashMap<String, LinkInfo>) -> String {
    let links: Vec<&str> = l_flag_value.split(",").collect();
    let mut s = String::new();

    for link in links {
        let link_parts: Vec<&str> = link.split("/").collect();
        let repo_abbrev = link_parts[0];
        let pr_id = link_parts[1];
        if config_links.contains_key(repo_abbrev) {
            let val = config_links.get(repo_abbrev).unwrap();
            s.push_str(&format!("- [{}]({}/{})\n", val.description, val.url, pr_id));
        }
    }

    s
}
