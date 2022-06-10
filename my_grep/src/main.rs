use std::env;
// use my_grep::{Config, run};
use config::core::{Config, run};

mod config;
fn main() {
	let args: Vec<String> = env::args().collect();
	
	let config = Config::new(&args).unwrap_or_else(|err| {
		println!("Проблема при разборе аргументов: {}", err);
		std::process::exit(1);
	});
	
	if let Err(e) = run(config) {
		println!("Ошибка в приложении: {}", e);
		std::process::exit(1);
	};	
}		
