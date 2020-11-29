use dirs;

mod cli;
mod comment;
mod config;

use comment::Comment;

fn main() {
    let args = cli::args();

    let home_path = dirs::home_dir().expect("can't get $HOME dir path");
    let config_file = config::path(home_path, config::FILE);
    let config = config::parse(config_file).expect("can't parse config file");

    let id = args.value_of("id").unwrap();
    let l = args.value_of("link").unwrap();
    let r = args.value_of("reviewer").unwrap_or("");
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
