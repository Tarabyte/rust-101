extern crate hello_world;
#[macro_use]
extern crate hello_world_derive;

use hello_world::HelloWorld;

struct Pancakes;

impl HelloWorld for Pancakes {
    fn hello_world() {
        println!("Hello World custom implementation for Pancakes");
    }
}

#[derive(Debug, HelloWorld)]
struct Coffee;

fn main() {
    Pancakes::hello_world();
    Coffee::hello_world();
}
