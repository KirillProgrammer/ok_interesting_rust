use std::{error::Error, fs};

pub struct Config {
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str>/* Config */ {
		if args.len() < 3 {
			// panic!("Нужно больше аргументов: строка поиска и имя файл")
			return Err("Нужно больше аргументов: строка поиска и имя файлa");
		}
		let query = args[1].clone();
		let filename = args[2].clone();

		// Config { query, filename }
		Ok(Config { query, filename })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;
	println!("Весь текст таков:\n{}", contents);
	Ok(())
}