use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[allow(unused_variables)]
fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");

    drop(b);

    println!("count after b goes out of scope = {}", Rc::strong_count(&a));
}
