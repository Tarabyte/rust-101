fn main() {
    test_box_values();

    test_list();

    test_deref();

    test_my_deref();

    test_drop();
}

fn test_box_values() {
    let boxed = Box::new(10);

    println!("{}, {:?}", boxed, boxed);
}

fn test_list() {
    use std::fmt::Debug;

    #[derive(Debug)]
    enum List<T>  {
        Nil,
        Cons(T, Box<List<T>>),
    }


    impl<T> List<T> where T: Debug {
        fn from_iterable<I> (iterator: I) -> Self
            where I: Iterator<Item = T>
        {
            iterator.fold(List::Nil, |acc, value| List::Cons(value, Box::new(acc)))
        }
    }

    println!("{:?}", List::from_iterable(1..10));
}

fn test_deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn test_my_deref() {
    use std::ops::Deref;

    #[derive(Debug)]
    struct MyBox<T> {
        data: T,
    }

    impl<T> MyBox<T> {
        fn new(data: T) -> Self {
            MyBox {
                data,
            }
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.data
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    println!("{:?}, {:?}", y, *y);
    assert_eq!(5, *y);

    fn test_deref_coercion(name: &str) {
        println!("Hello, {}!", name);
    }

    let name = MyBox::new("Rust".to_string());

    test_deref_coercion(&name);
}

fn test_drop() {
    #[derive(Debug)]
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`", self.data);
        }
    }

    impl CustomSmartPointer {
        fn new(data: String) -> Self {
            CustomSmartPointer { data, }
        }
    }

    let p = CustomSmartPointer::new("my stuff".to_string());

    println!("CustomSmartPointer created {:?}", p);

    {
        let _p = CustomSmartPointer::new("nested scope".to_string());
    }

    let early = CustomSmartPointer::new("early drop".to_string());

    std::mem::drop(early);
    println!("early should be already dropped");
}