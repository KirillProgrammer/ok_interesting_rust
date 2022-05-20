
// enum IpAddrKind {
// 	V4,
// 	V6,
// }

// struct IpAddr {
// 	kind: IpAddrKind,
// 	adress: String,
// }             Или

enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}


// Другой вариант

struct Ipv4Addr { }
struct Ipv6Addr { }

enum _IpAddr {
	V4(Ipv4Addr),
	V6(Ipv6Addr),
}

impl _IpAddr {
	fn call(&self) { }
	fn change(&self) { }
}


// 

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	Nebraske,
	OtherHer,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Монетка на счастье!");	
			1
		},
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("Монетка из штата {:#?}", state);
			25
		},
	}
}


// Перечисление Option<T>

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn main() {
	// let four = IpAddrKind::V4;
	// let sux = IpAddrKind::V6;
	// sux.route();

	// let home = IpAddr {
	// 	kind: IpAddrKind::V4,
	// 	adress: String::from("127.0.0.1"),
	// };
	// let loopback = IpAddr {
	// 	kind: IpAddrKind::V6,
	// 	adress: String::from("::1"),
	// };

	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));

	let quarter = Coin::Quarter(UsState::Nebraske);
	let coin = value_in_cents(&quarter);

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	// выражения if let
	
	let some_u8_value = Some(0u8);
	// match some_u8_value {
	// 	Some(3) => println!("три"),
	// 	_ => println!("None"),
	// }
	// Аналогично 
	if let Some(3) = some_u8_value {
		println!("три");
	}


	let mut count = 0;
	match quarter {
		Coin::Quarter(state) => println!("Четвертак из штата {:?}", state),
		_ => count +=1,
	}

	// Или 
	let quarter2 = Coin::Quarter(UsState::Alabama);
	let mut count1 = 0;
	if let Coin::Quarter(state) = quarter2 {
		println!("Четвертак из штата {:?}", state);
	} else {
		count1 += 1;
	}
}