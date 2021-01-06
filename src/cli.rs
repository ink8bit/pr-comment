//! Application info and args
//!
//! Defines:
//! - app name
//! - app version
//! - app description
//! - app author
//! - and all available app arguments

use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgMatches};

/// Provides app info and available args
pub fn args() -> ArgMatches {
    App::new("comment")
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
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

/// Returns an error if reviewer's nickname contains `@` sign
fn is_valid_name(val: &str) -> Result<(), &'static str> {
    if val.starts_with('@') {
        return Err("Reviewer nickname should be a string without '@' sign.");
    }
    Ok(())
}
