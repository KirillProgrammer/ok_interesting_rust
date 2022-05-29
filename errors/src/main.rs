#![allow(unused)]

use std::{fs::{File, self}, io::{self, Read, ErrorKind}};

fn main() {
    let mut f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
				ErrorKind::NotFound => 
					match File::create("hello.txt") {
						Ok(fileC) => fileC,
						Err(error) => panic!("Problems with file {:?}", error),
					},
				other_error => panic!("Panic! {:?}", other_error),				
			}
        },
    };
    let mut buffer = String::new();
    f.read_to_string(&mut buffer);
    println!("{:?}", buffer);

	// Вариант с использованием замыканий

	// let f = File::open("hello.txt").unwrap_or_else(|error| {
	// 	if error.kind() == ErrorKind::NotFound {
	// 		File::create("hello.txt").unwrap_or_else(|error| {
	// 			panic!("{}", error);
	// 		})
	// 	} else {
	// 		panic!("{}", error);
	// 	}
	// });

	// unwrap представляет собой укороченный метод, который реализован точно так же, как выражение match. Если
	// значение Result является вариантом Ok, то метод unwrap вернет значение содержащееся внутри Ok. 
	// Если же Result равно варианту Err, то метод unwrap вызовет макрокоманду panic!.

	let f = File::open("hello.txt").unwrap();

	// Еще один метод, expect, похожий на unwrap, позволяет помимо этого выбирать сообщение об ошибке
	//  макрокоманды panic!. Использование метода expect вместо unwrap и корректные сообщения об ошибках 
	// дают возможность более конкретно передать суть ошибки

	let f = File::open("hello.txt").expect("Не получилось открыть hello.txt");

}

// Вариант без распространения ошибок

// fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
// 	let f = File::open(filename);

// 	let mut f = match f {
// 		Ok(file) => file,
// 		Err(e) => return Err(e),
// 	};

// 	let mut s = String::new();

// 	match f.read_to_string(&mut s) {
// 		Ok(_) => Ok(s),
// 		Err(e) => Err(e),
// 	}
// }

// С распространением -------------------- 6 строк против 16

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
	// Если поместить оператор ? после значения типа Result, то он будет работать 
	// почти так же, как выражения match, которые мы определяли для обработки 
	// значений Result
	let mut f = File::open(filename)?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

// // Еще более кратко 

// fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
// 	let mut s = String::new();
// 	File::open(filename)?.read_to_string(&mut s)?;

// 	Ok(s)
// }

// // И еще

// fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
// 	fs::read_to_string(filename)
// }