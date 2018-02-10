fn main() {
    test_same_lifetime();

    test_shorter_lifetime();

    test_longer_lifetime();

    test_struct_with_ref();
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test_struct_with_ref() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };

    println!("Struct with ref {:?}", i);
}

fn test_same_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn test_shorter_lifetime() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn test_longer_lifetime() {
    let string1 = String::from("long string is long");
    let result;

    {
        let _string2 = String::from("xyz");
        result = &string1[..];
        //result = longest(string1.as_str(), _string2.as_str());
        println!("The longest string is {}", result);
    }

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}