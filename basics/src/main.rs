#[derive(Debug)]
struct Un {
    col: u32,
    row: u32,
    name: String,
}

impl Un {
    fn new(col: u32, row: u32, name: &str) -> Un {
        Un { col, row, name: String::from(name) }
    }
	fn clear(&mut self) {
		self.col = 0;
		self.row = 0;
		self.name = String::new();
	}
    fn wow_who_are_you() {
        println!("Wow who are you?");
    }
}

fn main() {
    let y = "uwu";
	let mut x = Un::new(1, 2, y);
	println!("{:?}", x);
	let m = x.clear();
	println!("{:?}", m);

    Un::wow_who_are_you();
}
