#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() // into_iter() берет во владение вектор, поэтому его после использования функции больше не будет
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let mut v2: Vec<i32> = vec![];

    for val in v1_iter {
        v2.push(*val);
        println!("{:?}", v2);
    }

    let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v3);
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("кроссовки"),
        },
        Shoe {
            size: 13,
            style: String::from("сандалии"),
        },
        Shoe {
            size: 10,
            style: String::from("ботинки"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("{:?}", in_my_size);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", sum);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // Простенькая имплементация итератора для своей структуры/типа
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
