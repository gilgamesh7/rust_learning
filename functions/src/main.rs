fn main() {
    println!("Hello, world!");
    print_phase();
    print_phrase("Processing data...");
    let result = gcd(48, 18);
    println!("The GCD of 48 and 18 is: {}", result);
    println!("Multiple return values example: {}", multiple_return_values(true));
    println!("Multiple return values example: {}", multiple_return_values(false));
}

fn print_phase(){
    println!("Phase 1: Initialization");
}

fn print_phrase(phrase: &str){
    println!("Phase 2: {}", phrase);
}

fn gcd(mut a:u64, mut b:u64) -> u64{
    while a !=0 {
        if a < b{
            let c:u64 = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b
}

fn multiple_return_values(flag:bool) -> bool {
    if flag {
        return true;
    }
    false
}