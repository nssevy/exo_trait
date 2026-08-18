#![allow(unused)]
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let pts: Point = Point { x: 3, y: 7 };
    let pts2: Point = pts.clone();

    println!("{:?}", pts);
    println!("{:?}", pts2);
}
