fn main() {
    let mut val: Vec<i32> = vec![1, 3, 5, 7];

    println!("{}", check_first_value(&val));

    val.push(15);

    println!("{:?}", val);

    let mut val_2: i8 = 10;
    println!("{}", val_2);
    println!("{}", add_two(val_2));
    println!("{}", val_2);
}

fn check_first_value(val: &Vec<i32>) -> bool {
    if val[0] == 1{
        return true;
    } 
    return false;
}

fn add_two(mut val: i8) -> i8{
    val += 2;

    println!("{}", val);

    return val;
}
