struct Car {
    make: String,
    model: String,
    year: i32,
}

fn main() {
    let my_box: Box<Car> = Box::new(Car {
        make: String::from("Honda"),
        model: String::from("Civic"),
        year: 2022,
    });

    print_car(&my_box);
    
    println!("Make: {}", my_box.make);
    println!("Model: {}", my_box.model);
    println!("Year: {}", my_box.year);
}

fn print_car(car: &Car) {
    println!("Make: {}", car.make);
    println!("Model: {}", car.model);
    println!("Year: {}", car.year);
}
