#[derive(Debug)]
struct Car {
    mpg: f32,
    colour: String,
    top_speed: u8,
}

impl Car {
    fn set_mpg(&mut self, mpg: f32) {
        self.mpg = mpg;
    }

    fn set_colour(&mut self, colour: String) {
        self.colour = colour;
    }

    fn set_top_speed(&mut self, top_speed: u8) {
        self.top_speed = top_speed;
    }
}

fn main() {
    let mut car = Car {
        mpg: 30.5,
        colour: String::from("Blue"),
        top_speed: 120,
    };

    println!("Car: {}, {}, {}", car.mpg, car.colour, car.top_speed);

    car.set_mpg(35.0);
    car.set_colour(String::from("Red"));
    car.set_top_speed(150);

    println!("Updated Car: {}, {}, {}", car.mpg, car.colour, car.top_speed);

    println!("Car : {:?}", car); // the trait `std::fmt::Debug` is not implemented for `Car`
}
