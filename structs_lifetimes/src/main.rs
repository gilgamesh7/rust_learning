struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32);

struct UnitStruct;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("vrbabu"),
        sign_in_count: 1,
    };

    println!("User: {}, Active: {}, Sign-in Count: {}", user1.username, user1.active, user1.sign_in_count);

    let user2 = build_user(String::from("vrbabu2"));
    println!("User: {}, Active: {}, Sign-in Count: {}", user2.username, user2.active, user2.sign_in_count);

    let coords = Coordinates(10, 20, 30);
    println!("Coordinates: ({}, {}, {})", coords.0, coords.1, coords.2);
}

fn build_user(username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 1,
    }
}
