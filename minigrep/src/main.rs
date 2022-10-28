use std::{env, process};
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("we're searching for `{}` in file: `{}`", config.query, config.filepath);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
