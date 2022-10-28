use std::{env, fs, error::Error};

pub struct Config {
	pub query: String,
	pub filepath: String,
	pub ignore_case: bool,
}

impl Config {
	// pub fn build(mut args: &[String]) -> Result<Config, &'static str> {
	// 	let args_len = args.len();
	// 	// let err = format!("Needed 3 arguments, but got {}", args_len);
	// 	if args_len < 3 {
	// 		return Err("Needed 3 arguments, but got less");
	// 	}

	// 	let query = &args[1];
	// 	let filepath = &args[2];

	// 	let ignore_case = env::var("IGNORE_CASE").is_ok();

	// 	Ok(Config {
	// 		query: query.to_string(),
	// 		filepath: filepath.to_string(),
	// 		ignore_case,
	// 	})
	// 	// Config { query, filepath }
	// }
	pub fn build(
		mut args: impl Iterator<Item = String>,
	) -> Result<Config, &'static str> {
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string"),
		};

		let filepath = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file path"),
		};

		let ignore_case = env::var("IGNORE_CASE").is_ok();

		Ok(Config {
			query: query.to_string(),
			filepath: filepath.to_string(),
			ignore_case,
		})
		// Config { query, filepath }
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.filepath)?;

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &content)
	} else {
		search(&config.query, &content)
	};

	for line in results {
		println!("{line}");
	}

	Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	// let mut results = vec![];
	// for line in content.lines() {
	// 	if line.contains(query) {
	// 		results.push(line)
	// 	}
	// }

	// results
	content.lines()
		.filter(|line| line.contains(query))
		.collect()

}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	// let mut results = Vec::new();

	// for line in content.lines() {
	// 	if line.to_lowercase().contains(&query) {
	// 		results.push(line);
	// 	}
	// }

	// results
	content
		.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()
}

pub struct TestData<'a> {
	pub query: Vec<&'a str>,
	pub content: &'a str,
}

impl TestData<'_> {
	pub fn new() -> TestData<'static> {
		TestData {
			query: vec!["duct", "RUSt"],
			content: "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.",
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let test_data = TestData::new();

		assert_eq!(vec!["safe, fast, productive."], search(test_data.query[0], test_data.content));
	}

	#[test]
	fn case_insensitive() {
		let test_data = TestData::new();
		let query = test_data.query[1];
		let content = test_data.content;

		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(&query, &content));
	}
}
