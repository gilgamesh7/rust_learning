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
}
