use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    test_file_open();

    let s = read_password_from_file().expect("File not found");

    println!("Password is {}", s);

    let s = read_username_from_file().expect("File not found");

    println!("Username is {}", s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_password_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match io::Read::read_to_string(&mut f, &mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn test_file_open() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            println!("Creating a file");

            match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!(
                    "Failed to create the file {:?}.",
                    e
                )
            }
        },
        Err(e) => panic!(
            "Something went wrong when opening the file {:?}.",
            e
        ),
    };
}
