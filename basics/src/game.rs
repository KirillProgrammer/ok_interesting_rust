use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Секретное число равно {}", secret_number);

    loop {
        println!("Введите свою догадку");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Не получилось прочитать строку");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Вы загадали {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Мало"),
            Ordering::Greater => println!("Много"),
            Ordering::Equal => {
                println!("То самое");
                break;
            }
        }
    }
}
    