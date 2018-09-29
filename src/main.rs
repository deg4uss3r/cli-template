extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("{{project-name}}")
        .version("1.0")
        .author("{{authors}}")
        .about("Howdy, Rust!")
        .arg(
            Arg::with_name("greeting")
                .short("g")
                .long("greeting")
                .default_value("world")
                .help("Sets a greeting for this hello program")
                .takes_value(true),
        )
        .get_matches();

    let greeting = matches.value_of("greeting").unwrap_or_default();
    println!("Hello, {}!", greeting);
}
