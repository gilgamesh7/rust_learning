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

fn main() {
    let my_rect = Rectangle { width: 10.0, height: 20.0 };
    let my_circle = Circle { radius: 10.0 };
    println!("Rectangle area: {}", my_rect.area());
    println!("Circle area: {}", my_circle.area());
}
