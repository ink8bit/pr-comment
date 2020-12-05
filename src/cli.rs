use clap::{App, Arg, ArgMatches};

pub fn args() -> ArgMatches {
    App::new("comment")
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
        )
        .arg(
            Arg::new("bug")
                .short('b')
                .long("bug")
                .about("Sets a bug value to true"),
        )
        .arg(
            Arg::new("copy")
                .short('c')
                .long("copy")
                .about("Copies comment to clipboard"),
        )
        .get_matches()
}
