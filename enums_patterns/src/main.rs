
#[derive(Debug)]
enum Pet{
    dog,
    cat,
    fish,
    turtle,
}

impl Pet{
    fn what_am_i(self) -> &'static str{
        match self{
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",
            Pet::turtle => "I am a turtle",
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

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn what_pet_is_it(pet: Pet) -> &'static str{
    match pet{
        Pet::dog => "I am a dog",
        Pet::cat => "I am a cat",
        Pet::fish => "I am a fish",
        _ => "I am some other pet"
    }
}

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

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    println!("five: {:?}, six: {:?}, none: {:?}", five.unwrap(), six.unwrap(), none);

    let my_dog = Pet::dog;
    println!("What pet is it? {}", what_pet_is_it(my_dog));

    let my_turtle = Pet::turtle;
    println!("What pet is it? {}", what_pet_is_it(my_turtle));

    let dog2: Option<Pet> = Some(Pet::dog);
    if let Some(Pet::dog) = dog2 {
        println!("It's a dog!");
    } else {
        println!("It's not a dog.");
    }

    let dog2: Option<Pet> = Some(Pet::cat);
    if let Some(Pet::dog) = dog2 {
        println!("It's a dog!");
    } else {
        println!("It's not a dog.");
    }

    let mut stack: Vec<i8> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
}
