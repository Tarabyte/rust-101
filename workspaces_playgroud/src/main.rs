extern crate add_one;


fn main() {
    let x = 2;
    println!("{} + 1 = {}", x, add_one::add_one(x));
}
