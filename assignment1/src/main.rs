fn main() {
    let val1: u8 = 5;
    let val2: u8 = 2;

    let ans:u8 = val1 % val2;
    println!("The answer is: {}", ans);

    let mut vec_1: Vec<u8> = vec![2, 4, 6, 8, 10];
    println!("The vector is: {:?}", vec_1);
    vec_1.pop();
    println!("The vector is: {:?}", vec_1);
    vec_1.push(12);
    println!("The vector is: {:?}", vec_1);

    let my_str: String = String::from("Hello");
    println!("Concated String is: {}", concat_string(my_str, String::from(" World!")));

    println!("Control flow result is: {}", control_flow(1));
    println!("Control flow result is: {}", control_flow(60));
    println!("Control flow result is: {}", control_flow(20));
    println!("Control flow result is: {}", control_flow(30));

}

fn concat_string(str1: String, str2: String) -> String {
    let mut new_str: String = str1.clone();
    new_str.push_str(&str2);

    return new_str;
}

fn control_flow(my_int: i32)->String{
    if my_int == 1 {
        return String::from("The value is one");
    } else if my_int > 50 {
        return String::from("The value is greater than 50");
    } else if my_int < 25{
        return String::from("The value is less than 25");
    } else if (my_int > 25) && (my_int < 50){
        return String::from("The value is greater than 25 but less than 50");
    } else {
        return String::from("The value is 25 or 50");
    }
}