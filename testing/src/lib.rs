#[derive(Debug, PartialEq)]
pub struct Rectangle {
	length: u32,
	width: u32,
}

impl Rectangle {
	pub fn can_hold(&self, other: &Rectangle) -> bool {
		self.length > other.length && self.width > other.width
	}
}

pub fn add_two(a: i32) -> i32 {
	a + 2
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn larger_can_hald_smaller() {
		let larger = Rectangle { length: 8, width: 7 };
		let smaller = Rectangle { length: 5, width:1 };

		assert!(larger.can_hold(&smaller));
    }
	#[test]
	fn smaller_cannot_hald_larger() {
		let larger = Rectangle { length: 8, width: 7 };
		let smaller = Rectangle { length: 5, width:1 };

		assert!(!smaller.can_hold(&larger));
    }
	#[test]
	fn it_adds_two() {
		assert_eq!(4, add_two(2));
	}
	#[test]
	fn it_ne_adds_two() {
		assert_ne!(5, add_two(4)); 
	}
	#[test]
	fn rects() {
		let rec1 = Rectangle { width: 2, length: 1 };
		let rec2 = Rectangle { width: 1, length: 3 };
		assert_ne!(rec1, rec2);
	}
}
