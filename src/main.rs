#[macro_use] extern crate quicli;

use quicli::prelude::*;

// Add cool slogan for your app here, e.g.:
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "greeting", short = "g", default_value = "world")]
    greeting: String,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    println!("Hello, {}!", args.greeting);
});
