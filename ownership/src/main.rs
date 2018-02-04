fn main() {
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", s);

    double_refs();

    clone_data();

    clone_primitive();

    // taking ownership
    let s = String::from("originally owned by main");

    take_ownership(s);

    // copying Copyable
    let x = 5;
    copy_primitive(x);
    println!("x is still here {}", x);

    // ownership back and force
    test_ownership_passing();

    // returning a tuple
    test_returning_tuple();

    // passing references
    test_passing_by_ref();

    // mutable references
    test_mutable_references();

    // slices
    test_slices();
}

fn test_slices() {
    let s = String::from("Hello 1  you there");

    let first = first_word(&s[..]);

    let second = second_word(&s[..]);

    println!("The first word of '{}' is '{}'", s, first);
    println!("The second word of '{}' is '{}'", s, second);
}

fn first_word(s: &str) -> &str {
    for (index, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    return &s[..];
}

fn second_word(s: &str) -> &str {
    let mut start = 0;
    let mut end = s.len();

    for (index, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            if start > 0 {
                end = index;
                break;
            } else {
                start = index
            }
        }
    }

    &s[start..end]
}


fn test_mutable_references() {
    let mut s1 = String::from("Some mutable string");

    {
        println!("{}", s1);
        let s1_ref = &mut s1;

        append_yo(s1_ref);
        append_yo(s1_ref);

        // println!("{}", s1);
        println!("{}", *s1_ref);
    }


    println!("{}", s1);
}

fn append_yo(s: &mut String) {
    s.push_str(" yo!")
}

fn test_passing_by_ref() {
    let s1 = String::from("Some string");
    let len = compute_length_by_ref(&s1);

    println!("s1 = '{}', len = {}", s1, len);
}

fn compute_length_by_ref(s: &String) -> usize {
    s.len()
}

fn test_returning_tuple() {
    let s1 = String::from("some string");
    let (len, s1) = compute_length(s1);

    println!("s1 = '{}', len = {}", s1, len);
}

fn compute_length(s: String) -> (usize, String) {
    (s.len(), s)
}

fn test_ownership_passing() {
    let s1 = give_me_ownership();

    let s2 = String::from("local");

    let s3 = take_ownership_and_give_back(s2);

    println!("s1 '{}', s3 '{}'", s1, s3);
}

fn give_me_ownership() -> String {
    String::from("Give you ownership")
}

fn take_ownership_and_give_back(s: String) -> String {
    s
}

fn double_refs() {
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}", s2); // ok
    // println!("{}", s1); // fail
}

fn clone_data() {
    let s1 = String::from("clonable");
    let mut s2 = s1.clone();

    s2.push_str("/cloned");

    println!("original \"{}\" clone \"{}\"", s1, s2);
}

fn clone_primitive() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn take_ownership(s: String) {
    println!("I own this string '{}'", s);
}

fn copy_primitive(x: u32) {
    println!("This is just a copy of int {}", x);
}