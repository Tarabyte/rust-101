fn main() {
    println!("Hello, world!");

    test_pattern_matching();

    test_option_matching();

    test_if_let_syntax();
}

fn test_if_let_syntax() {
    let some_value: Option<i32> = None;

    if let Some(x) = some_value {
        println!("Some value is {}", x);
    } else {
        println!("Some value is None")
    }
}

fn test_option_matching() {
    let some_value: Option<i32> = Some(10);

    fn double(x: i32) -> i32 {
        x * 2
    }

    fn double_option(x: &Option<i32>) -> Option<i32> {
        match *x {
            Some(value) => Some(double(value)),
            _ => None,
        }
    }

    println!(
        "Value {:?} doubled {:?}",
        some_value,
        double_option(&some_value)
    );
}

fn test_pattern_matching() {
    #[derive(Debug)]
    enum Coin {
        Penny, Nickel, Dime, Quarter,
    }

    impl Coin {
        fn to_cents(&self) -> u8 {
            match *self {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
    }

    let coin = Coin::Dime;

    println!("The value of {:?} is {}", coin, coin.to_cents());
}