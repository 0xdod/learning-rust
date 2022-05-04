#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// fn hello(name: &str) {
//     println!("Hello, {}", name);
// }

// PREVENTING REFERENCE CYCLES WITH WEAK REFERENCES
/**
 * We want a Node to own its children, and we want to share that
ownership with variables so we can access each Node in the tree
directly. To do this, we define the Vec<T> items to be values of type
Rc<Node>. We also want to modify which nodes are children of
another node, so we have a RefCell<T> in children around the
Vec<Rc<Node>>.

We want a Node to own its children, and we want to share that
ownership with variables so we can access each Node in the tree
directly. To do this, we define the Vec<T> items to be values of type
Rc<Node>. We also want to modify which nodes are children of
another node, so we have a RefCell<T> in children around the
Vec<Rc<Node>>.

 */
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    // using the box pointer to allocate data on the heap
    //  let b = Box::new(5);
    // println!("b = {}", b);
    // to demonstrate deref coercion
    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    // let value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    //At this point, when we try to get a reference to the parent of leaf by
    // using the upgrade method, we get a None value
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![vec![Rc::clone(&leaf)]]),
        parent: RefCell::new(Weak::new()),
    });
    // we can modify leaf to give it a Weak<Node> reference to its parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    //When we print the parent of leaf again ➎, this time we’ll get a
    // Some variant holding branch: now leaf can access its parent!
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
