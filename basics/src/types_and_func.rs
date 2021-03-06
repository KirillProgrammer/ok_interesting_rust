fn main() {
  // Кортежи
    let tup = (500, 2.1, 2);
    let tupac: (i32, f64, u8) = (13434, 234.34, 57);
    // можно деструктуризировать как в js
    let (x, y, z) = tup;
    // можно обращаться к элементам через .
    let fuve = tupac.0;
    let u = fuve as f64 + x as f64 + y + z as f64;
    println!("{}", u.to_string());

    let a = [1, 2, 3, 4, 5]; // все эллементы массива должны иметь один и тот же тип в отличии от кортежа. в отличии от него Вектор может уменьшаться и увеличиваться в размере, их рассмотрим по книге позже
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // можно указывать тип и кол-во эллементов
    let bs = [3; 5]; // будет создан массив вида [3, 3, 3] аналогично записи let bs = [3, 3, 3]
    let first = b[3]; // можно обращаться к эллементам по индексу
    fn another_function(s: &str) -> &str {
        println!("WoW wHo ArE yOu? "); // используется snake_case
        println!("I am {}", s);
        if s.ends_with("d") {
            println!("Suck");
        } else {
            println!("Good");
        }
        "Good" // выражение без точки с запятой аналогично вызову return
    }

    let mat = another_function("Bond");
    println!("{}", mat);

    let uyk = 6;

    let yuk = {
        let uyk = 4;
        uyk + 1 // аналогично return uyk + 1;
    }; // то есть мы присвоили переменной значение вычисленное внутри блока, используя локальную переменную и игнорируя внешнюю

    fn five() -> i32 {
        5
    } // функция через -> указывает возвращаемое значение и не добавляя в конце выражения ; мы будто используем ключевое слово return
    let number = 3;
    if number < 5 { // условие обязательно должно быть типа bool, тут не поддерживается как в js что непустая строка или число не равное нулю - true, а остальное - false
        println!("True");
    } else if number >20 {
        println!("WhoAH");
    } else {
        println!("Not true");
    }

    let condition = true;

    let numca = if condition {
        5
    } else {
        6
    }; // Можно использовать условные выражения для присвоения значений переменным
    let ru = String::from("hello world");
    let hello = &ru[0..5];
    let world = &ru[6..]; // что-то типа реактивных данных в JS. Если изменится строка, то изменятся и переменные
    let mour = first_word(&ru[..]);
    println!("{}", mour);
    let ciur = "geasdf";
    let msas = first_word(&ciur);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
