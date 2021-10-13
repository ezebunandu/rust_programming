use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Sam Ezebunandu <sam.ezebunandu@gmail.com>")
        .about("echo program written in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("Do not print newline")
                .takes_value(false)
                .short("n"),
        )
        .get_matches();
    println!("{:#?}", matches);
}
