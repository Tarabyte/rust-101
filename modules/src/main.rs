pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("Running");
            }
        }
    }
}

use a::series::of;
use a::series::of::nested_modules;

fn main() {
    a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();

    test_partial();
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn test_partial() {
    use TrafficLight::{Red, Yellow};

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("{:?} {:?} {:?}", red, yellow, green);
}