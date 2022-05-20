mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn take_payment() {}
    }
}


fn serve_order() {}

mod back_of_house {
	fn cook_order() {}

	fn fix_incorrect_order() {
		cook_order();
		super::serve_order();
	}
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruits: String,
	}

	pub enum Appetizer {
		Soup,
		Salad,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast { 
				toast: String::from(toast), 
				seasonal_fruits: String::from("персики") 
			}
		}
	}
}

pub fn eat_at_restaurant() {
	// Абсолютный путь
	crate::front_of_house::hosting::add_to_waitlist();

	// Относительный путь
	front_of_house::hosting::add_to_waitlist();

	let mut meal = back_of_house::Breakfast::summer("ржаной");
	meal.toast = String::from("пшеничный");
	println!("I want {} toast, please", meal.toast);
	// А вот менять не публичные поля нельзя
	// meal.seasonal_fruits = String::from("bluberry");

	let order1 = back_of_house::Appetizer::Salad;
	let order2 = back_of_house::Appetizer::Soup;
}