fn main() {
    test_appending_string();

    test_plus();

    test_format();

    test_iteration();
}

fn test_iteration() {
    let hi = "Здравствуйте".to_string();

    for byte in hi.bytes() {
        println!("{}", byte)
    }

    for char in hi.chars() {
        println!("{}", char);
    }
}

fn test_format() {
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();

    let tic_tac_toe = format!("{}-{}-{}", s1, s2, s3);

    println!("{} + {} + {} = {}", s1, s2, s3, tic_tac_toe);
}

fn test_plus() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{}, {}", s2, s3);
}

fn test_appending_string() {
    let mut s = "foo".to_string();
    let s2 = "bar";

    s.push_str(&(&(&s2)));

    println!("s {} and s2 {}", s, s2);
}