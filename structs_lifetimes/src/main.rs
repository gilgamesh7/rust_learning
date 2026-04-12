struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

struct Coordinates(i32, i32, i32);

struct UnitStruct;

struct Square{
    width: u32,
    height: u32,
}

impl Square{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn whats_my_width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

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

    let sq = Square { width: 5, height: 10 };
    println!("Square: width {}, height {}", sq.width, sq.height);
    println!("Area of the square: {}", sq.area());
    println!("Width of the square: {}", sq.whats_my_width());

    let mut sq2 = Square { width: 3, height: 4 };
    println!("Square 2: width {}, height {}", sq2.width, sq2.height);
    sq2.change_width(6);
    println!("Square 2 after width change: width {}, height {}", sq2.width, sq2.height);

    let r;
    {
        let x = 5;
        r = &x;
    }
    // println!("Value of r: {}", r); // This will cause a compile-time error because r is referencing x which has gone out of scope.

    let s = String::from("hello");
    let r = example_for_lifetime_annotation(&s);
    println!("Value of r: {}", r);

}

fn example_for_lifetime_annotation<'a>(x: &'a str) -> &'a str{
    x
}
fn build_user(username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 1,
    }
}
