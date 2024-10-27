fn main() {
    println!("Hello, world!");

    use_optionenum_iterator_patternmatching();
}

fn use_optionenum_iterator_patternmatching() {
    let numbers = vec![5, 2, 8, 1, 3, 7];

    let max_element = numbers.iter().max();

    match max_element {
        Some(&max) => println!("The maximum element is {0}", max),
        None => println!("The vector is empty"),
    }
}
