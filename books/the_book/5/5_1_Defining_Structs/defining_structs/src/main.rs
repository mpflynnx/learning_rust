// A User struct definition
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // no semi-colon needed in Rust

// field init shorthand syntax
// https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html#using-the-field-init-shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand syntax
        email, // field init shorthand syntax
        sign_in_count: 1,
    }
}

fn main() {
    // Creating an instance of the User struct
    // NOTE: mutability is a property of the struct instance
    // binding, meaning that if the instance is 'mut' all its
    // fields are mutable
    let mut user1 = User {
        active: true,
        username: String::from("usernumber234"),
        email: String::from("usernumber234@example.com"),
        sign_in_count: 1,
    };

    // Changing the value in the email field of a User instance
    user1.email = String::from("newuseremail@example.com");

    println!("user1: {:?}", user1);

    // new instance from other instance
    // Struct update syntax
    let user2 = User {
        username: String::from("user2name"),
        email: String::from("user2email@email.com"),
        ..user1 // all other fields the same as user1
                // user1 still valid as remaining fields are types that implement the copy trait
    };

    println!("user2: {:?}", user2);

    // new instance from other instance
    // invalidates user1 as user1 username moved to user3
    let user3 = User {
        email: String::from("user3email@email.com"),
        ..user1 // all other fields the same as user1 including username??
                // invalidates user1 as user1 username moved to user3
    };

    println!("user3: {:?}", user3);
    // println!("user1: {:?}", user1); // compile error value borrowed here after partial move

    let user4 = build_user(
        String::from("newuser238@email.com"),
        String::from("newuser238"),
    );

    println!("user4: {:?}", user4);
}
