fn main() {
    // The range of i8 is from -128 to 127
    let x:i8 = -127;
    println!("The value of x is: {}", x);

    // The range of u8 is from 0 to 255
    let y:u8 = 255;
    println!("The value of y is: {}", y);

    let decimal = 02_55;
    println!("The value of decimal is: {}", decimal);

    let hexadecimal:u32 = 0xFF;
    println!("The value of hexadecimal is: {}", hexadecimal);

    let octal:u32 = 0o377;
    println!("The value of octal is: {}", octal);

    let binary:u32 = 0b11111111;
    println!("The value of binary is: {}", binary);

    let byte:u8 = b'Z';
    println!("The value of byte is: {}", byte);

    // Floatig points
    let a:f32 = 3.14;
    println!("The value of a is: {}", a);

    let b = 9.8;
    println!("The value of b is: {}", b);

    // Boolean
    let c:bool = true;
    println!("The value of c is: {}", c);

    // Character
    let d:char = 'R';
    println!("The value of d is: {}", d);

    // Tuple
    let e:(i32, f64, char) = (500, 6.4, 'A');
    println!("The value of e is: {:?}", e);
    println!("The first value of e is: {}", e.0);
    println!("The second value of e is: {}", e.1);
    println!("The third value of e is: {}", e.2);

    let (x,y,z) = e;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let a:char = e.2;
    println!("The value of a is: {}", a);

    // Array
    let f:[i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of f is: {:?}", f);
    println!("The value of f is: {:#?}", f);
    // println!("The value of f is : {}", f); the trait `std::fmt::Display` is not implemented for `[i32; 5]`
    println!("The first value of f is: {}", f[0]);
    println!("The second value of f is: {}", f[1]);
    println!("The third value of f is: {}", f[2]);
    println!("The fourth value of f is: {}", f[3]);
    println!("The fifth value of f is: {}", f[4]);

    let mut array: [i32;5] = [1,2,3,4,5];
    println!("The value of array is: {:?}", array);
    array[0] = 10;
    println!("The value of array is: {:?}", array);

    // Vector
    let mut g:Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("The value of g is: {:?}", g);
    g.push(6);
    println!("The value of g is: {:?}", g);

    let mut h:Vec<char> = vec!['R', 'u', 's', 't'];
    println!("The value of h is: {:?}", h);
    h.pop();
    println!("The value of h is: {:?}", h);

    let mut j:Vec<i8> = Vec::new();
    j.push(1);
    j.push(2);
    j.push(3);
    println!("The value of j is: {:?}", j);

    let mut k = Vec::<i8>::with_capacity(2);
    k.push(1);
    k.push(2);
    println!("The value of k is: {:?}", k);
    println!("The capacity of k is: {}", k.capacity());
    k.push(3);
    println!("The capacity of k is: {}", k.capacity());

    // iterators
    let l:Vec<i32> = (0..5).collect();
    println!("The value of l is: {:?}", l);

    // Slice
    let m:Vec<i8> = (0..5).collect();
    println!("The value of m is: {:?}", m);

    let sm: &[i8] = &m;
    println!("The value of sm is: {:?}", sm);
    println!("Pointer : {:p}", sm.as_ptr());
    println!("Length : {}", sm.len());

    let sm2: &[i8] = &m[1..3];
    println!("The value of sm2 is: {:?}", sm2);

    use std::mem;
    println!("Size of slice reference: {}", mem::size_of_val(&sm));
    println!("Size of normal reference: {}", mem::size_of::<&i8>());
}
