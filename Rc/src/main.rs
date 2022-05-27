use std::rc::Rc;

struct Owner {
    name: String,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn main() {
    // Создает "владельца" с подсчётом ссылок 
    let gadget_owner: Rc<Owner> = Rc::new(
        Owner {
            name: "Gadget Man".to_string(),
        }
    );

    // Создание "гаджета" принадлежащего "gadget_owner". Клонирование "Rc<Owner>"
    // дает нам новый указатель на все того же владельца выделенного в памяти и увеличивает
    // количкство ссылок в процессе
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    // Удаляем локальную переменную "владельца гаджетов"
    drop(gadget_owner);

    // Несмотря на удаление "владельца гаджетов" мы можем напечатать имя "владельца"
    // этих "гаджетов". Это все потому, что мы удалили тольк один "Rc<Owner>" а не "Owner" на
    // которого он указывает. До тех пор пока "Rc<Owner>" указывает хоть на одного выделенного в
    // памяти "Owner" он будет храниться в памяти. Проекция поля "gadget1.owner.name" работает
    // потому что "Rc<Owner>" автоматически разыменовывается в "Owner"
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    // В конце функции main "gadget1" и "gadget2" уничтожаются и с ними подсчитанные ссылки на
    // "Owner" уничтожаются также. Владелец вместе со всеми гаджетами теперь полностью уничтожен
}
