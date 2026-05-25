struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}
fn main() {
    let coord = Point{x: 5, y: 10};
    let coord2 = Point{x: 'x', y: 'y'};

    let coord3 = Point2{x: 5, y: 'y'};
}
