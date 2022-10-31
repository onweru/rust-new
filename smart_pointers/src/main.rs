use std::ops::Deref;

fn main() {
    // println!("Hello, world!");
    int_on_heap();

    recursive_list();

    play_with_pointers();

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn int_on_heap() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn recursive_list() -> List {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list
}

fn play_with_pointers() {
    let x = 5;
    let _y = &x; // same as 👇
    let _y = Box::new(x); // same as👇
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}!`", self.data);
    }
}