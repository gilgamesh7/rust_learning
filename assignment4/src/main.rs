enum Shape {
    Square(f64),          // side length
    Pentagon(f64),        // side length
    Octagon(f64),         // side length
    Triangle(f64, f64),   // base, height
}   

impl Shape {
    fn corners(&self) -> i8{
        match self {
            Shape::Square(_) => 4,
            Shape::Pentagon(_) => 5,
            Shape::Octagon(_) => 8,
            Shape::Triangle(_, _) => 3,
        }
    }
}

fn main() {
    let square = Shape::Square(5.0);
    let pentagon = Shape::Pentagon(3.0);
    let octagon = Shape::Octagon(2.0);
    let triangle = Shape::Triangle(4.0, 6.0);

    println!("Square corners: {}", square.corners());
    println!("Pentagon corners: {}", pentagon.corners());
    println!("Octagon corners: {}", octagon.corners());
    println!("Triangle corners: {}", triangle.corners());
}
