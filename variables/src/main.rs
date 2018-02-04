const MAX_POINTS: u32 = 1_0_0_01_0;

fn main() {
    // mutable
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // constant
    println!("MAX_POINTS is {}", MAX_POINTS);


    // shadowing

    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // chars
    let cat: char = '😻';
    println!("Hello {}", cat);

    // tuples
    let tup = (1, 1.1, 'c');

    // println!("Can we print tuple {}", tup); no we can't
    println!("Print tuple elements {}, {}, {}", tup.0, tup.1, tup.2);

    // destructuring
    // always  need to destructure all
    let (_x, _, _) = tup;

    let (x, y, _) = tup;

    println!("x {}, y {}", x, y);

    // arrays
    let a = [1, 2, 3, 4, 5];
    println!("How to get array length {}", a.len());
    println!("How to access items by index first {}, last {}", a[0], a[a.len() - 1]);

    // println!("Can we print entire array {}", a); nope

    let a_slice = &a[1..4];
    println!("Slice length {}", a_slice.len());
}
