use clap::{App, Arg};

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

    let matches = app.get_matches();
    println!("{:?}", matches);
}
