use std::env;
// use my_grep::{Config, run};
use config::core::{run, Config};

mod config;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Проблема при разборе аргументов: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Ошибка в приложении: {}", e);
        std::process::exit(1);
    };
}
