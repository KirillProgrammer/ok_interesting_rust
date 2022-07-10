use crate::List::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!(
        "родительский узел leaf = {:?}",
        leaf.parent.borrow().upgrade()
    );

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "родительский узел leaf = {:#?}",
        leaf.parent.borrow().upgrade()
    );

    /*    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));*/
    /*println!("a начальное число rc = {}", Rc::strong_count(&a));*/
    /*println!("a следующий элемент = {:?}", a.tail());*/

    /*let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));*/
    /*println!("a число rc после создания b = {}", Rc::strong_count(&a));*/
    /*println!("b начальное число rc = {}", Rc::strong_count(&b));*/
    /*println!("b следующий элемент = {:?}", b.tail());*/

    //if let Some(link) = a.tail() {
    //*link.borrow_mut() = Rc::clone(&b);
    //}

    //println!("b число rc после изменения a = {}", Rc::strong_count(&b));
    //println!("a число rc после изменения a = {}", Rc::strong_count(&a));

    // Раскомментируйте следующую строку кода, и вы увидите, что у нас цикл;
    // он переполнит стек.
    //println!("a следующий элемент = {:?}", a.tail());
}
