use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Sam Ezebunandu <sam.ezebunandu@gmail.com>")
        .about("echo program written in Rust")
        .get_matches();
}
