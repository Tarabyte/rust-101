use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T> List<T> {
    fn from<I>(iter: I) -> Self
        where I: Iterator<Item = T>
    {
        iter.fold(List::Nil, |list, value| List::Cons(value, Rc::new(list)))
    }

    fn prepend(self, value: T) -> Self {
        List::Cons(value, Rc::new(self))
    }
}

fn main() {
    test_list_impl();

    test_multiple_owning();
}

fn test_list_impl() {
    let list = List::from(1..10);

    println!("{:?}", list);
}

fn test_multiple_owning() {
    let a = List::Nil.prepend(1).prepend(2);

    let a_ref = Rc::new(a);
    println!("a_ref strong count {}", Rc::strong_count(&a_ref));
    let b = List::Cons(3, Rc::clone(&a_ref));
    println!("a_ref strong count {}", Rc::strong_count(&a_ref));
    let c = List::Cons(4, Rc::clone(&a_ref));
    println!("a_ref strong count {}", Rc::strong_count(&a_ref));

    println!("{:?}, {:?}", b, c);

}