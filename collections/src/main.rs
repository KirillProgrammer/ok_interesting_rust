fn main() {
    // Векторы
	let _v = vec![1, 2, 3]; // аннотация типа не требуется, выводится автоматически
	
	let mut v: Vec<i32> = Vec::new(); // требуется аннотация типа, так как язык не знает, какие данные мы будем хранить в векторе
	
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);
	{
		let v = vec![0, 1, 2, 3, 4];
		let u = &v[1];
		println!("{}", u);
		
		match v.get(1) {
			Some(u) => println!("Второй элемент равен {}", u),
			None => println!("Второй элемент отсутствует"),
		}
	}

	// let does_not_exist = &v[100];  Такое обращение к несуществующему элементу вызовет панику и программа завершится
	let _does_not_exist = v.get(100); // Такое обращение к несузествующему эллементу вернет None и не выйдет из программы

	let mut v = vec![1, 2, 4, 4, 5];
	let first = &v[0];

	v.push(6);
	let first = &v[0]; // Если заново не переопределить указатель на первый элемент массива, то будет ошибка, так как при увеличении вектора на один элемент он мог сдвинутся в другое место в памяти, а значит и старый указатель указывает не туда, куда надо

	println!("Первый элемент равен {}", first);

	// Перебор значений 

	for i in &v {
		print!("{}", i);
	}
	println!();
	// Перебор изменяемых ссылок

	for i in &mut v {
		*i *= 2; // Для того чтобы изменить значение, на которое ссылается изменяемая ссылка, приходится использовать оператор разыменования (*), чтобы получить значение в i перед тем, как использовать оператор *=
		print!("{}", i);
	}
	println!();

	// Хранение в векторе значений разного типа, что бывает довольно удобно, а реализуется всего лишь через Перечисления

	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Float(10.12),
		SpreadsheetCell::Text(String::from("синий")),
	];

	// Строки

	let mut s = String::new();
	let data = "first letter";
	let s = data.to_string();
	// аналогично
	let mut s = "first letter".to_string();

	s.push_str(" mommy");

	let mut s1 = String::from("foo");
	let s2 = "bar";
	s1.push_str(s2); // push_str принимает только &str 
	s1.push('l'); 		 // push добавляет в конец символ типа char
	println!("s2 равна {}", s2);

	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2; // s1 перенесено сюда и больше не может использоваться, а s2 передается по указателю 

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");
	let s = format!("{}-{}-{}", s1, s2, s3);
	// код с использованием макрокоманды format! гораздо легче читается и не берет во владение ни один из своих параметров.
	let hello = "Здравствуйте";
	let s = &hello[0..4];
	println!("{s}");

	for c in "Здравствуйте".chars() {
		print!("{c} ");
	}
	println!();
	for c in "Здравствуйте".bytes() {
		print!("{c} ");
	}
	println!();

	// Хеши

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Red"), 21);
	scores.insert(String::from("Blue"), 50); // Перекрыли прошлое значение для Blue равное 10

	let teams = vec![String::from("Blue"), String::from("Red")];
	let initial_score = vec![10,50];
	let results: HashMap<_, _> = teams.iter().zip(initial_score.iter()).collect();
	println!("{:#?}", results);

	let field_name = String::from("Favorite colour");
	let field_value = String::from("Blue");
	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	// field_name и field_value дальше не доступны, так как они перешли во владение HashMap 

	let team_name = scores.get(&"Blue".to_owned());
	println!("{:?}", team_name);

	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}

	let mut garden = HashMap::new();

	garden.insert(String::from("Rosesa"), 10);
	garden.insert(String::from("Rimash"), 11);

	garden.entry(String::from("Rosesa")).or_insert(87);
	garden.entry(String::from("Tulpan")).or_insert(10);

	println!("{:#?}", garden);

	let text = "здравствуй мир чудесный мир";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	
	println!("{:?}", map);

	let vy = vec![5,3,5,4,4,5,2,5,4,2,4,4,5,4,5,5];
	let mut count = 0u32;
	for i in &vy {
		count += *i as u32;
	}
	println!("{count}");
	println!("{}", vy.len());
	println!("{}", (count as f64 / vy.len() as f64));

}
