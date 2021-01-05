use clap::{App, Arg, ArgMatches};

pub fn args() -> ArgMatches {
    App::new("comment")
        .version("0.0.2")
        .about("comment is a CLI app which creates a formatted comment for your pull requests.")
        .author("ink8bit")
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
                .takes_value(true)
                .validator(is_valid_name),
        )
        .arg(
            Arg::new("copy")
                .short('c')
                .long("copy")
                .about("Copies comment to clipboard"),
        )
        .get_matches()
}

fn is_valid_name(val: &str) -> Result<(), String> {
    if val.starts_with('@') {
        return Err(String::from(
            "Reviewer nickname should be a string without '@' sign.",
        ));
    }
    Ok(())
}
