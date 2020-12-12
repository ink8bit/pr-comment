use clipboard::{ClipboardContext, ClipboardProvider};

mod cli;
mod comment;
mod config;

use comment::Comment;
use config::Config;

fn main() {
    let args = cli::args();
    let config = Config::new().expect("Couldn't parse config file.");
    let id = args.value_of("id").unwrap();
    let l = args.value_of("link").unwrap();
    let r = args.value_of("reviewer").unwrap_or("");
    let is_bug = args.is_present("bug");
    let need_copy = args.is_present("copy");

    let comment = Comment {
        id: id.to_string(),
        links: l.to_string(),
        reviewers: r.to_string(),
        config,
        is_bug,
    };

    let output = Comment::new(comment).expect("Could not create comment.");

    println!("{}", output);

    if need_copy {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(output).unwrap();
    }
}
