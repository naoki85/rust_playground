use std::rc::Rc;
use List::{Cons, Nil};
// use std::ops::Deref;

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
//
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data: {}", self.data);
//     }
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("list = {:?}", list);
    // let x = 5;
    // let y = MyBox::new(x);
    //
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };
    // println!("CustomSmartPointer created!");
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // drop(c);
    // println!("CustomSmartPointer dropped before the end of main.");
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
