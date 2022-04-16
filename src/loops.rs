fn main() {
    let mut a = 0;
    loop { // аналогично while true в других языках - бесконечный цикл
        println!("еще раз!");
        a = a + 1;
        if a > 3 {
            break;
        }
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Значение равно {}", element);
    }
    for num in 1..4 {
        println!("{}!", num);
    }
    fn fibonacci(num: i32) -> i32 {
        if num == 1 {
            return 0;
        } else if num == 2 {
            return 1;
        } else {
            return fibonacci(num - 1) + fibonacci(num - 2);
        }
    }

    let u = fibonacci(8);
    let u = u.to_string();
    println!("{}!", u);
}
