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
}
