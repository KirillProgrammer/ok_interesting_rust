use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_spec_value = 10;
	let simulated_rand_num = 7;

	generate_workout(
		simulated_user_spec_value,
		simulated_rand_num
	);

}

fn generate_workout(intensity: u32, rand_num: u32) {
	let mut expensive_result= Cacher::new(|num| {
		println!("вычисляется медленно...");
		thread::sleep(Duration::from_secs(1));
		num
	});
	// let expensive_closure = |num: u32| -> u32 {
	// 	println!("вычисляется медленно...");
	// 	thread::sleep(Duration::from_secs(1));
	// 	num
	// };
	if intensity < 25 {
		println!(
			"Сегодня сделайте {} отжиманий",
			expensive_result.value(intensity)
		);
		println!(
			"Далее сделайте {} приседаний",
			expensive_result.value(intensity)
		);
	} else {
		if rand_num == 3 {
			println!("Сделайте сегодня перерыв");
		} else {
			println!(
				"Сегодня пробежка {} минут",
				expensive_result.value(intensity)
			);
		}
	}
}

struct Cacher<T>
	where T: Fn(u32) -> u32,
{
	calculation: T,
	value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
	where T: Fn(u32) -> u32
{
	fn new(calculation: T) -> Cacher<T> {
		Cacher {
			calculation,
			value: HashMap::new(),
		}
	}
	
	fn value(&mut self, arg: u32) -> u32 {
		let kar = self.value.get(&arg);
		match kar {
			Some(k) => *k,
			None => {
				let v = (self.calculation)(arg);
				self.value.insert(v, v);
				v
			},
		}

	}
}
