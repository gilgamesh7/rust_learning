use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}


impl<T> Add for Point<T>
where
    T: Add<Output = T>{
        type Output = Self;
        fn add(self, rhs:Self)->Self{
            Point{
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

struct Point2<T, U> {
    x: T,
    y: U,
}

trait Overview {
    fn overview(&self) -> String{
            String::from("This is an overview.")
    }
}

struct Course {
    headline : String,
    author : String,
}

struct AnotherCourse {
    headline : String,
    author : String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("Dropping Course: {}", self.author);
    }
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

fn main() {
    let coord = Point{x: 5, y: 10};
    let coord2 = Point{x: 'x', y: 'y'};

    let coord3 = Point2{x: 5, y: 'y'};

    let course1 = Course {
        headline: String::from("Rust for Beginners"),
        author: String::from("John Doe"),
    };

    let course2 = AnotherCourse {
        headline: String::from("Advanced Rust"),
        author: String::from("Jane Smith"),
    };

    println!("{}", course1.overview());
    println!("{}", course2.overview());


    call_overview(&course1);
    call_overview(&course2);
    call_overview_generic(&course1);
    call_overview_generic(&course2);

    // drop(course1); //Will still be called when it goes out of scope

    let coord3 = Point{x: 5, y: 10};
    let coord4 = Point{x: 15, y: 20};   

    let sum = coord3 + coord4;  
    println!("Sum of coordinates: ({:?})", sum);


}

fn call_overview(item: &impl Overview) {
    println!("{}", item.overview());
}

fn call_overview_generic<T: Overview>(item: &T) {
    println!("{}", item.overview());
}