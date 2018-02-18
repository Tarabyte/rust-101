use std::rc::{Rc, Weak};
use std::cell::RefCell;
use List::{Cons, Nil};


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new_leaf(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }

    fn new_branch(value: i32, leaf: &mut Rc<Node>) -> Rc<Node> {
        let branch = Node::new_leaf(value);

        branch.children.borrow_mut().push(Rc::clone(leaf));

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        branch
    }
}

fn main() {
    test_cycle_with_strong_refs();

    test_tree_like_structures();

    test_play_with_ref_cell();

    test_get_result_from_thread();
}

fn test_cycle_with_strong_refs() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(ref link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will
    // overflow the stack
    // println!("a next item = {:?}", a.tail());
}

fn test_tree_like_structures() {
    let mut leaf = Node::new_leaf(3);

    let branch = Node::new_branch(5, &mut leaf);

    println!("Leaf before adding to branch {:?}", leaf);

    println!(
        "Branch with a leaf {:?}, {}",
        branch,
        branch.children.borrow().len()
    );

    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());
}

fn test_play_with_ref_cell() {
    let v: RefCell<Vec<String>> = RefCell::new(vec![
        "Hello".to_string(),
        "World".to_string()
    ]);

    v.borrow_mut().push("!".to_string());

    println!("{:?}", v.borrow());

    *v.borrow_mut() = vec!["Replaced".to_string()];

    println!("{:?}", v.borrow());
}

fn test_get_result_from_thread() {
    use std::thread;
    use std::time::Duration;

    let result = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let c = RefCell::new(5);
        let _m = c.borrow_mut();

        let _b = c.borrow(); // this causes a panic
    }).join();

    println!("{:?}", result)
}