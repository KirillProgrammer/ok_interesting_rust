use std::collections::HashMap;

struct Cacher<T, V>
	where T: Fn(V) -> V,
{
	calculation: T,
	value: HashMap<V, V>,
}

impl<T, V> Cacher<T, V>
	where T: Fn(V) -> V,
	V: Eq + std::hash::Hash,
	V: Eq + std::hash::Hash + Copy
{
	fn new(calculation: T) -> Cacher<T, V> {
		Cacher {
			calculation,
			value: HashMap::new(),
		}
	}
	
	fn value(&mut self, arg: V) -> V {
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

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn call_with_dif_val() {
		let mut cache = Cacher::new(|a| a);

		let v1 = cache.value(1);
		let v2 = cache.value(2);

		assert_ne!(v1, v2);
		assert_eq!(v2, 2);
	}
	// #[test]
	// fn cache_strings() {
	// 	let mut cache = Cacher::new(|a| a);

	// 	let v1 = cache.value("hello".to_string());
	// 	let v2 = cache.value("bye".to_string());

	// 	assert_ne!(v1, v2);
	// 	assert_eq!(v2, "bye".to_string());
	// }
	#[test]
	fn cache_str() {
		let mut cache = Cacher::new(|a| a);

		let v1 = cache.value("hello");
		let v2 = cache.value("bye");

		assert_ne!(v1, v2);
		assert_eq!(v2, "bye");
	}
	// #[test]
	// fn cache_float() {
	// 	let mut cache = Cacher::new(|a| a);

	// 	let v1 = cache.value(123.32);
	// 	let v2 = cache.value(2.31);

	// 	assert_ne!(v1, v2);
	// 	assert_eq!(v2, 2);
	// }
}