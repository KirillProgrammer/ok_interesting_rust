#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn new_square(size: u32) -> Self {
        Self { width: size, height: size }
    }
    fn new_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width: width, height: height }
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50, };
    let rect1 = Rectangle { width: 10, height: 40, };
    let rect2 = Rectangle { width: 60, height: 45, };

    println!(
        "Площадь прямоугольника равна {} квадратным пикселам",
        rect.area()
    );
    println!("Может ли поместиться rect в rect1? {}", 
                            rect.can_hold(&rect1));
    println!("Может ли поместиться rect1 в rect2? {}", 
                            rect1.can_hold(&rect2));
    // let pool = area(&rect);
    println!("{:#?}", rect);

    let mimir = Rectangle::new_square(8);

    println!("{:#?}", mimir);
}
