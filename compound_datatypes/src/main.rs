fn main() {
    tuple_example();
    array_example();
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
}