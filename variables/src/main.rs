fn main() {
    let x = 5;
    println!("The Value of x is : {}",x);

    // x = 6; // This will cause an error because x is immutable by default
    // println!("The Value of x is : {}",x);

    let mut y = 10; // This makes y mutable
    println!("The Value of y is : {}",y);
    y = 15; // Now we can change the value of y
    println!("The Value of y is : {}",y);

    const SECONDS: i8 = 60; // This is a constant, it must be annotated with a type and cannot be changed
    println!("Seconds in a minute: {}", SECONDS);

    
}

