//! comment is a CLI utility to create a string comment for your pull request and copy it to clipboard.

#[cfg(target_os = "macos")]
mod clipboard;

mod cli;
mod comment;
mod config;

use comment::Comment;
use config::Config;

fn main() {
    let args = cli::args();
    let links = args.value_of("link").unwrap();
    let reviewers = args.value_of("reviewer").unwrap_or("");

    #[cfg(target_os = "macos")]
    let need_copy = args.is_present("copy");

    let config = Config::new().expect("Couldn't parse config file.");

    let output = Comment::new(reviewers, links, config).expect("Could not create comment.");

    let printed = output.print();
    println!("{}", &printed);

    #[cfg(target_os = "macos")]
    match clipboard::copy(need_copy, printed) {
        Ok(v) => println!("{}", v),
        Err(e) => eprintln!("{}", e),
    }
}
