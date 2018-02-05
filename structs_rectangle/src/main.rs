fn main() {
    test_separate_variables();

    test_tuples();

    test_structs();

    test_structs_with_method();
}

fn test_structs_with_method() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            let Rectangle { width, height } = *self;
            width * height
        }

        fn includes(&self, rect: &Rectangle) -> bool {
            rect.width < self.width && rect.height < self.height
        }

        fn square(width: u32) -> Rectangle {
            Rectangle {
                width,
                height: width
            }
        }
    }


    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Rectangle {:?} area is {}",
        rect,
        rect.area()
    );

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!(
        "{:?} includes {:?}? {}",
        rect,
        rect2,
        rect.includes(&rect2)
    );

    println!(
        "Square {:?}",
        Rectangle::square(20)
    );
}

fn test_structs() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "Rectangle {:?} area is {}",
        rect,
        area(&rect)
    );

    fn area(rect: &Rectangle) -> u32 {
        let Rectangle { width, height } = *rect;
        width * height
    }
}

fn test_tuples() {
    let rectangle = (30, 50);

    println!(
        "Rectangle area is {}",
        area(&rectangle)
    );

    fn area(rect: &(u32, u32)) -> u32 {
        rect.0 * rect.1
    }
}

fn test_separate_variables() {
    let width = 30;
    let height = 50;

    println!(
        "Rectangle area is {}",
        area(width, height)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}
