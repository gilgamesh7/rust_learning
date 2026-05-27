struct Point<T> {
    x: T,
    y: T,
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
}