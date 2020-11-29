use clap::{App, Arg};
use dirs;

mod comment;
mod config;

use comment::Comment;

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
    let config_file = config::path(home_path, config::FILE);
    let config = config::parse(config_file).expect("can't parse config file");

    let matches = app.get_matches();
    let id = matches.value_of("id").unwrap();
    let l = matches.value_of("link").unwrap();
    let r = matches.value_of("reviewer").unwrap_or("");
    let dr = config.default_reviewer;
    let rs = comment::reviewers(r, dr).expect("can't create a list of reviewers.");
    let ls = comment::links(l, config.links);

    let c = comment::create(Comment {
        id: id.to_string(),
        links: ls,
        reviewers: rs,
    });

    println!("{}", c);
}
