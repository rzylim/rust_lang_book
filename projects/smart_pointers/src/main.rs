use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

// use crate::List1::{Cons1, Nil1};
use crate::List2::{Cons2, Nil2};
// use crate::List3::{Cons3, Nil3};
use crate::List4::{Cons4, Nil4};
use crate::List5::{Cons5, Nil5};
use crate::List6::{Cons6, Nil6};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let list: List1 = Cons1(1, Cons1(2, Cons1(3, Nil1)));
    let list: List2 = Cons2(1, Box::new(Cons2(2, Box::new(Cons2(3, Box::new(Nil2))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(5, y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop();
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // let a = Cons3(5, Box::new(Cons3(10, Box::new(Nil3))));
    // let b = Cons3(3, Box::new(a));
    // let c = Cons3(4, Box::new(a));

    let a = Rc::new(Cons4(5, Rc::new(Cons4(10, Rc::new(Nil4)))));
    let b = Cons4(3, Rc::clone(&a));
    let c = Cons4(4, Rc::clone(&a));

    let a = Rc::new(Cons4(5, Rc::new(Cons4(10, Rc::new(Nil4)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons4(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons4(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons5(Rc::clone(&value), Rc::new(Nil5)));

    let b = Cons5(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons5(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let a = Rc::new(Cons6(5, RefCell::new(Rc::new(Nil6))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons6(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// enum List1 {
//     Cons1(i32, List),
//     Nil1,
// }

enum List2 {
    Cons2(i32, Box<List2>),
    Nil2,
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

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum List3 {
    Cons3(i32, Box<List3>),
    Nil3,
}

enum List4 {
    Cons4(i32, Rc<List4>),
    Nil4,
}

#[derive(Debug)]
enum List5 {
    Cons5(Rc<RefCell<i32>>, Rc<List5>),
    Nil5,
}

#[derive(Debug)]
enum List6 {
    Cons6(i32, RefCell<Rc<List6>>),
    Nil6,
}

impl List6 {
    fn tail(&self) -> Option<&RefCell<Rc<List6>>> {
        match self {
            Cons6(_, item) => Some(item),
            Nil6 => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
