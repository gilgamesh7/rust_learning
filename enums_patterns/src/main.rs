
#[derive(Debug)]
enum Pet{
    dog,
    cat,
    fish
}

impl Pet{
    fn what_am_i(self) -> &'static str{
        match self{
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish"
        }
    }
}

enum IpAddrKind{
    v4,
    v6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

// enum Option<T>{
//     None,
//     Some(T),
// }

fn main() {
    let my_dog = Pet::dog;
    println!("{:?}", my_dog);
    println!("{}", my_dog.what_am_i());

    let home = IpAddr{
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr{
        kind: IpAddrKind::v6,
        address: String::from("::1")
    };

    let some_number = Some(5);
    let some_string = Some("a string"); 
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y; // error[E0277]: cannot add `Option<i32>` to `i32`
    let sum = x + y.unwrap(); // This will work, but be careful with unwrap() as it can panic if y is None  
    println!("The sum is: {}", sum);
    
}
