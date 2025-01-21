fn main() {
    tuple_example();
    array_example();
    memory_safety_example();
}

fn tuple_example() {
    let tup : (i32, f64, u8)= (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    println!("The value of x is: {0}", tup.0);
}

fn array_example() {
    let a : [i32; 5]= [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    let a :[i32;5] = [3; 5];
    println!("The value of a is: {:#?}",a);
    println!("The value of a is: {:?}",a);
}

use std::io;
fn memory_safety_example() {
    // Entering amn index that is out of bounds will result in a panic
    // thread 'main' panicked at src/main.rs:43:25:
    // index out of bounds: the len is 5 but the index is 6
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    let a : [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index : String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element : i32 = a[index];
    println!(
        "The value of the element at index {index} is: {element}"
    );
}