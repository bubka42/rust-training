use std::num;

pub struct Point {
    x: f32,
    y: f32,
}

pub struct Circle {
    center: Point,
    radius: f32,
}

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

pub struct Rectangle {
    p1: Point,
    p2: Point,
}

pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

/// Compute area of shape
pub fn area(shape: Shape) -> u64 {
    match shape {
        Point => 0,
        Circle { center, radius } => std::f32::consts::PI * radius * radius,
        Triangle { p1, p2, p3 } => num::abs(p1.x(p2.y - p3.y) + p2.x(p3.y - p1.y) + p3.x(p1.y - p2.y)),
        Rectange { p1, p2 } => num::abs((p1.x - p2.x) * (p1.y - p2.y)),
    }
}

/// Compute perimeter of shape
pub fn perimeter(triang: Triangle) -> (u64, u32) {
    match shape {
        Point => 0,
        Circle { center, radius } => 2 * std::f32::consts::PI * radius,
        Triangle { p1, p2, p3 } => {
            let x1 = p1.x - p2.x;
            let y1 = p1.y - p2.y;
            let x2 = p2.x - p3.x;
            let y2 = p2.y - p3.y;
            let x3 = p3.x - p1.x;
            let y3 = p3.y - p1.y;
            (x1**2 + y1**2).sqrt() + (x2**2 + y2**2).sqrt() + (x3**2 + y3**2).sqrt()
        }
        Rectange { p1, p2 } => 2 * (num::abs(p1.x - p2.x) + num::abs(p1.y - p2.y)),
}
