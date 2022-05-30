use std::fmt::Display;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
	let result = largest(&number_list);
	println!("Наибольшее число равно {}", result);

	let char_list = vec!['y', 'm', 'a', 'q'];
	let result = largest(&char_list);
	println!("Наибольший символ равен {}", result);

	let tweet = Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("конечно, как вы, наверное, уже знаете, люди"),
		reply: false,
		retweet: false,
	};
	println!("1 новый твит: {}", tweet.summarize());
}


fn largest<T>(list: &[T]) -> T 
	where T: PartialOrd + Copy
{
	let mut largest = list[0];

	for &item in list.iter() {
		if item > largest {
			largest = item;
		}
	}

	largest
}

struct Point<T, V> {
	x: T,
	y: V,
}

impl<T, V> Point<T, V> {
	fn x(&self) -> &T {
		&self.x
	}
}

// после ключевого слова impl приходится объявлять T, в результате чего можно использовать его для уточнения, что мы реализуем методы в типе Point<T>. Благодаря объявлению T как обобщенного типа после impl, Rust будет отождествлять тип в угловых скобках в Point как обобщенный, а не конкретный.

pub trait Summary {   // Пока что походит просто на интерфейс, в котором мы объявляем обязательные для реализации функции
	fn summarize(&self) -> String;
	fn localize(&self) -> String { // реализация метода поумолчанию
		String::from("перевод будет, обещаю")
	}
}

pub struct NewsArticle {
	headline: String,
	location: String,
	author: String,
	content: String,
}

pub struct Tweet {
	username: String,
	content: String,
	reply: bool,
	retweet: bool,
}

impl Summary for NewsArticle {
	fn summarize(&self) -> String {
		format!("{}, {}, ({})", self.headline, self.location, self.author)
	}
}

impl Summary for Tweet {
	fn summarize(&self) -> String {
		format!("{}: {}", self.username, self.content)
	}
}

pub fn notify(item: impl Summary) { // эта функция принимает в аргумент любой тип
									// данных, имплементирующий трейт Summary
	println!("Bad News! {}", item.localize())
}

// pub fn notify<T: Summary>(item: T) {			аналогично функции выше, просто
// 	println!("Bad News! {}", item.localize())	другой вид записи, и фактически
// }											функция выше - просто сахар

// В определении notify мы указываем, что item должен реализовать 
// как Display, так и Summary

pub fn r_notify(item: impl Summary + Display) {}
// или 
// pub fn notify<T: Summary + Display>(item: T) {}


// Если много границ типажей, то можно так
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

// Но лучше вот так
fn some_function<T, U>(t: T, u: U) -> i32
	where T: Display + Clone,
	U: Clone + Summary
{
	32i32
}

fn returns_summarizable() -> impl Summary { // мы также можем отметить как возвращаемое значение любой тип, реализующий типаж Summary 
	Tweet {
		username: String::from("horse_ebooks"),
		content: String::from("конечно, как вы, наверное, уже знаете, люди"),
		reply: false,
		retweet: false,
	}
}