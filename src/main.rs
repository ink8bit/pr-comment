use clap::App;

fn main() {
    App::new("comment")
        .version("0.0.1")
        .about("Creates PR comment")
        .author("ink8bit")
        .get_matches();
}
