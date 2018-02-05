struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8);

fn main() {
    let mut user1 = User {
        username: String::from("Basil"),
        email: String::from("test@test.com"),
        active: true,
        sign_in_count: 0
    };

    user1.active = !user1.active;

    println!("{} is {}", user1.username, if user1.active {"active"} else {"passive"});

    let user2 = build_new_user(String::from("email"), String::from("Peter"));

    println!("Ping {} via {}", user2.username, user2.email);

    // test updates
    test_updates();
    test_inplace_updates();

    // tuple like
    test_tuple_like_structs();
}


fn test_tuple_like_structs() {
    let red = Color(255, 0, 0);

    println!("RGB {}, {}, {}", red.0, red.1, red.2);
}

fn test_inplace_updates() {
    let mut user = build_new_user(
        String::from("email"),
        String::from("username")
    );

    sign_in_user_by_ref(&mut user);
    sign_in_user_by_ref(&mut user);
    sign_in_user_by_ref(&mut user);

    println!("{} sign in {} times", user.username, user.sign_in_count)
}

fn sign_in_user_by_ref(user: &mut User) {
    user.sign_in_count = user.sign_in_count + 1;
}

fn test_updates() {
    let tarabyte = build_new_user(
        String::from("email"),
        String::from("tarabyte")
    );

    println!("{} sign in {} times", tarabyte.username, tarabyte.sign_in_count);

    let tarabyte = sign_in_user(tarabyte);

    println!("{} sign in {} times", tarabyte.username, tarabyte.sign_in_count);
}

fn build_new_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}

fn sign_in_user(user: User) -> User {
    User {
        sign_in_count: user.sign_in_count + 1,
        ..user
    }
}