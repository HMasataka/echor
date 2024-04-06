use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("1.0.0")
        .author("example@email.com")
        .about("Rust echo")
        .get_matches();
}
