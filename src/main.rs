use clap::{App, Arg};
use dirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::exit;

const CONFIG_FILE: &str = ".commentrc1";

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(rename(deserialize = "defaultReviewer"))]
    default_reviewer: String,
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

    let home_path = dirs::home_dir().expect("can't get $HOME dir path");
    let home_str_path = home_path.to_str().unwrap();
    let config_path = format!("{}/{}", home_str_path, CONFIG_FILE);
    let config_data = fs::read_to_string(config_path).expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_data).unwrap();
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

    let mut reviewer = r;
    if r.is_empty() {
        reviewer = &dr;
    }

    let template = format!(
        "
**PR**
`feature/{}`

**LINKS**
{}
- (repo name 1)[]
- (repo name 2)[]

**REVIEW**
{}
- @person_nickname_1
- @person_nickname_2

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
",
        id, l, reviewer,
    );

    println!("{}", template);
}

fn config_path() {
    todo!()
}

fn parse_config() {
    todo!()
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
