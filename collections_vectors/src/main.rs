fn main() {
    test_reading_elements();

    test_creating_from_slice();

    test_iteration();

    test_iteration_with_mutation();
}

fn test_iteration_with_mutation() {
    let mut v = vec![1, 2, 3];

    for item in &mut v {
        let prev = *item;

        *item = *item * 100;

        println!("{} -> {}", prev, item);
    }

    println!("{:?}", v);
}

fn test_iteration() {
    let v = vec![100, 300, 200];

    for item in &v {
        println!("{}", item)
    }

    println!("{:?}", v);
}

fn test_creating_from_slice() {
    use std::ops::Range;

    let v: Vec<Range<i32>> = vec![1..3, 2..3];

    println!("{:?}", v)
}

fn test_reading_elements() {
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];

    println!(
        "Third element of {:?} is {}",
        v,
        third
    );

    let does_not_exists = v.get(100);

    if let None = does_not_exists {
        println!("There is no 100th element is {:?}", v)
    }

    if let Some(x) = v.get(3) {
        println!(
            "The fourth element is {}",
            x
        )
    }
}
