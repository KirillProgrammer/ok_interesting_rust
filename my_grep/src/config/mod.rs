/*	
На примере этого файла примерно разобрался как организовывать модули внутри одного проекта,
теперь получается все более наглядно
*/


pub mod core {
	use std::{error::Error, fs};

	#[derive(Debug, PartialEq)]
	pub struct Config {
		pub query: String,
		pub filename: String,
		pub case_sensitive: bool,
	}

	impl Config {
		pub fn new(args: &[String]) -> Result<Config, &'static str>/* Config */ {
			if args.len() < 3 {
				// panic!("Нужно больше аргументов: строка поиска и имя файл")
				return Err("Нужно больше аргументов: строка поиска и имя файлa");
			}
			let mut case_sensitive = true;
			// = std::env::var("CASE_INSENSITIVE").is_err();
			for arg in args {
				if arg == "-ncs" || arg == "--no-case-sensitive" {
					case_sensitive = false;
				}
			}
			let query = args[1].clone();
			let filename = args[2].clone();

			// Config { query, filename }
			Ok(Config { query, filename, case_sensitive })
		}
	}

	pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
		let contents = fs::read_to_string(config.filename)?;

		let result = if config.case_sensitive {
			search(&config.query, &contents)
		} else {
			search_case_insensitive(&config.query, &contents)
		};

		for line in result {
			println!("{}", line);
		}
		Ok(())
	}

	pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
		let mut results: Vec<&'a str> = Vec::new();
		for line in contents.lines() {
			if line.contains(query) {
				results.push(line);
			}
		}
		if results.len() > 0 {
			return results;
		} else {
			return vec!["Не найдено совпадений"];
		}
	}
	pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
		let query = query.to_lowercase();
		let mut results = Vec::new();

		for line in contents.lines() {
			if line.to_lowercase().contains(&query) {
				results.push(line);
			}
		}

		results
	}
}

#[cfg(test)]
mod tests {
	use super::core::*;
	#[test]
	fn new_config() {
		let config = Config { 
			query: String::from("search"), 
			filename: String::from("b.txt"),
			case_sensitive: true,
		};

		let args = 
			vec![String::from("process"), String::from("search"), String::from("b.txt")];

		let real_config = Config::new(&args).unwrap_or_else(|err| {
			panic!("Проблема при разборе аргументов: {}", err);
			
		});

		assert_eq!(config, real_config);
	}
	#[test]
	fn test_run() {
		let args = 
			vec![String::from("process"), String::from("search"), String::from("search.txt")];
		let conf = Config::new(&args).unwrap_or_else(|err| {
			panic!("Failed to read args, {err}");
		});
		let done = run(conf).unwrap_or_else(|err| {
			panic!("Failed to run, {err}");
		});
		assert_eq!(done, ());
	}

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
		";
		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}
	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
	
}