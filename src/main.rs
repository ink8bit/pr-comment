use clipboard::{ClipboardContext, ClipboardProvider};

mod cli;
mod comment;
mod config;

use comment::Comment;

fn main() {
    let args = cli::args();

    let config_file = config::path(config::FILE);
    let config = config::parse(config_file).expect("Can't parse config file.");

    let id = args.value_of("id").unwrap();
    let l = args.value_of("link").unwrap();
    let r = args.value_of("reviewer").unwrap_or("");
    let is_bug = args.is_present("bug");
    let need_copy = args.is_present("copy");
    let dr = config.default_reviewer;

    let branch_name = comment::branch(id, is_bug);
    let revs = comment::reviewers(r, dr).expect("Can't create a list of reviewers.");
    let ls = comment::links(l, config.links);

    let c = comment::create(Comment {
        branch_name,
        links: ls,
        reviewers: revs,
    });

    println!("{}", c);

    if need_copy {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(c).unwrap();
    }
}
