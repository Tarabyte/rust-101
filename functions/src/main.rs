fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_arg(15);

    another_function_with_2_args(1, 2);

    // block is exprssion
    let y = {
        let x = 1;
        x + 1
    };

    println!("The value of block is {}", y);

    println!("Five is {}", five());

    println!("Add 1 to 5 = {}", add_1(5));
}

fn add_1(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_arg(x: i32) {
    println!("The value of x is {}", x);
}

fn another_function_with_2_args(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}