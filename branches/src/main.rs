fn main() {
    let number = 3;

    let res: i32 = if number < 5 {
        println!("condition was true");
        5
    } else {
        println!("condition was false");
        -5
    };

    println!("If is expression {}", res);

    // omitting else
    let _partial = if number < 3 {
        5;
    };

    // println!("Partial if {}", _partial); partial is ()

    multiple_branches();

    // loops
    looping();

    looping_while();

    for_array();

    for_range();

    for_array_with_index();
}

fn multiple_branches() {
   let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn looping() {
    let mut x = 0;
    loop {
        println!("looping {}!", x);

        x = x + 1;

        if x > 9 {
            break;
        }
    }
}

fn looping_while() {
    let mut number = 3;

    while number != 0 {
        println!("While {}!", number);

        number = number - 1;
    }
}

fn for_array() {
    let array = [1, 2, 3, 4, 5];

    for item in array.iter() {
        println!("Item is {}", item);
    }
}

fn for_range() {
    for item in (1..5).rev() {
        println!("Item in range {}", item);
    }
}

fn for_array_with_index() {
    let array = 1..6;

    for (i, item) in array.enumerate() {
        println!("array[{}] is {}", i, item);
    }
}