use std::collections::HashMap;

pub struct Cacher<T, V>
	where T: Fn(V) -> V,
{
	calculation: T,
	value: HashMap<V, V>,
}

impl<T, V> Cacher<T, V>
	where T: Fn(V) -> V,
	V: Eq + std::hash::Hash,
	V: Eq + std::hash::Hash + Clone
{
	pub fn new(calculation: T) -> Cacher<T, V> {
		Cacher {
			calculation,
			value: HashMap::new(),
		}
	}
	
	pub fn value(&mut self, arg: V) -> V {
		let key = arg.clone();
		let value = (self.calculation)(arg);
		let func_return = value.clone();
		self.value.entry(key).or_insert(value);
		func_return
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn cache_nums() {
		let mut cache = Cacher::new(|a| a);

		let v1 = cache.value(1);
		let v2 = cache.value(2);

		assert_ne!(v1, v2);
		assert_eq!(v2, 2);
	}
	#[test]
	fn cache_strings() {
		let mut cache = Cacher::new(|a| a);

		let v1 = cache.value("hello".to_string());
		let v2 = cache.value("bye".to_string());

		assert_ne!(v1, v2);
		assert_eq!(v2, "bye".to_string());
	}
	#[test]
	fn cache_str() {
		let mut cache = Cacher::new(|a| a);

		let v1 = cache.value("hello");
		let v2 = cache.value("bye");

		assert_ne!(v1, v2);
		assert_eq!(v2, "bye");
	}
}