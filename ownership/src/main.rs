fn main() {
    let var_1 = 1; // On the stack
    let var_2 = String::from("Hello"); // On the heap

    let x = vec!["Hello".to_string()]; // On the heap
    let y = x; // Move ownership of the vector to y
    // println!("{:?}", x); // This will cause a compile-time error because x is no longer valid
    println!("{:?}", y); // This will work because y now owns the vector

    // Clone
    let x = vec!["Hello".to_string()]; // On the heap
    let y = x.clone(); // Clone the vector, creating a new one on the heap
    println!("{:?}", x); // This will work because x is still valid
    println!("{:?}", y); // This will also work because y is a clone of x

    // Copy
    let x = 1; // On the stack
    let y = x; // Copy the value of x to y
    println!("{}", x); // This will work because x is still valid
    println!("{}", y); // This will also work because y is a copy of x - implementscopy trait

    let s = String::from("Hello"); // On the heap
    takes_ownership(s); // Give ownership to function - s's value moves into the function and is no longer valid
    // println!("{}", s); // This will cause a compile-time error because s is no longer valid

    let my_int: i32 = 5; // On the stack
    makes_copy(my_int); // i32 implements copy trait - Copy the value of my_int to the function - my_int is still valid after the function call
    println!("{}", my_int); // This will work because my_int is still valid

    let s1: String = gives_ownership(); // gives_ownership moves its return value into s1
    println!("{}", s1); // This will work because s1 now owns the string

    let s2: String = String::from("Hello"); // On the heap
    let s3: String = takes_and_gives_back(s2); // s2 is moved into the function and then returned, moving ownership to s3
    println!("{}", s3); 
    // println!("{}", s2); // This will cause a compile-time error because s2 is no longer
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    return some_string // Return the string and move ownership to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string; // Return the string and move ownership back to the caller
}