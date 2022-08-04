use std::env;
use std::process;

// TODO(ding.wang): figure out this
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // TODO(ding.wang): figure out this
    if let Err(e) = minigrep::run(config) {
        eprintln!("{}", e)
    }
}
