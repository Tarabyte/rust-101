macro_rules! my_vec {
    ($( $x:expr ),* ) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
}

fn main() {
    let vector = my_vec![1, 2, 3];

    println!("{:?}", vector);
}
