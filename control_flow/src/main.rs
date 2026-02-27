fn main() {
    println!("Hello, world!");

    let one = 1;
    if one > 0 {
        println!("one is greater than zero");
    } else if one == 1{
        println!("one is equal to one");
    }
    else {
        println!("one is not greater than zero");
    }

    let mut counter = 0;
    loop {
        println!("This will loop forever");
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    let mut loop_counter = 5;
    'counter: loop{
        println!("This will loop forever {} times", loop_counter);
        loop_counter -= 1;
        let mut decrease = 5;
        loop {
            println!("Decreasing from 5: {}", decrease);
            decrease -= 1;
            if decrease == 0 {
                break 'counter;
            }
        }
    }

    while loop_counter != 0 {
        println!("Loop counter is not zero: {}", loop_counter);
        loop_counter -= 1;
    }

    let vec: Vec<i8> = (0..5).collect();
    for element in vec {
        println!("Element: {}", element);
    }

    for number in (1..4).rev() {
        println!("Number: {}", number);
    }

    println!("{:?}", (1..4).rev());
}
