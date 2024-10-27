trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,  
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Generic Shape implementation
fn print_area<T: Shape>(shape: &T) {
    println!("Shape area: {0}", shape.area());
}

// Generic
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let my_rect = Rectangle { width: 10.0, height: 20.0 };
    let my_circle = Circle { radius: 10.0 };
    println!("Rectangle area: {}", my_rect.area());
    println!("Circle area: {}", my_circle.area());

    print_area(&my_rect);
    print_area(&my_circle);

    // Generics on point
    let int_point = Point { x: 10, y: 20 };
    let float_point = Point { x: 10.2, y: 20.4 };
    println!("Point x {0}, y {1}", int_point.x, int_point.y);
    println!("Point x {0}, y {1}", float_point.x, float_point.y);
}
