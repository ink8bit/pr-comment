#[cfg(target_os = "macos")]
use clipboard::{ClipboardContext, ClipboardProvider};

mod cli;
mod comment;
mod config;

use comment::Comment;
use config::Config;

fn main() {
    let args = cli::args();
    let id = args.value_of("id").unwrap();
    let links = args.value_of("link").unwrap();
    let reviewers = args.value_of("reviewer").unwrap_or("");
    let is_bug = args.is_present("bug");

    #[cfg(target_os = "macos")]
    let need_copy = args.is_present("copy");

    let config = Config::new().expect("Couldn't parse config file.");

    let output =
        Comment::new(id, reviewers, links, is_bug, config).expect("Could not create comment.");

    let printed = output.print();
    println!("{}", &printed);

    #[cfg(target_os = "macos")]
    if need_copy {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(printed).unwrap();
    }
}
