
use std::fmt::Display;

fn main() {
    // let string1 = String::from("abcd");
	// let string2 = "xyz";
	// let result = longest(string1.as_str(), string2);
	// println!("Самая длинная строка равна {}", result);

	let string1 = String::from("длинная строка является длинной");
	let result;
	{
		let string2 = String::from("xyz");
		result = longest(string1.as_str(), string2.as_str());
	}
	println!("Самая длинная строка равна {}", result);

	let novel = String::from("Зовите меня Измаил. Много лет тому назад...");
	let first_sentence = novel.split('.')
		.next()
		.expect("Не смог отыскать точечку");
	let i = ImportantExcerpt { part: first_sentence };
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		"y"
	}
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
	part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
	fn level(&self) -> u32 {
		3
	}
	fn announve_and_return_part(&self, announcment: &str) -> &str {
		println!("uWu: {}", announcment);
		self.part
	}
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
	where T: Display
{
	println!("WOW: {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}