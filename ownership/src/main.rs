// fn main() {
//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's value moves into the function...
//     // ... and so is no longer valid here

//     let x = 5; // x comes into scope

//     makes_copy(x); // because i32 implements the Copy trait,
//     // x does NOT move into the function,
//     println!("{}", x); // so it's okay to use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
// // special happens.

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let mut user1 = build_user(email, username);
    user1.email = String::from("anotheremail@example.com");

    let mut user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
