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
pub fn area(shape: Shape) -> f32 {
    match shape {
        Shape::Point(_) => 0.0,
        Shape::Circle(Circle { center, radius }) => std::f32::consts::PI * radius * radius,
        Shape::Triangle(Triangle { p1, p2, p3 }) => {
            (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)).abs() / 2.0
        }
        Shape::Rectangle(Rectangle { p1, p2 }) => ((p1.x - p2.x) * (p1.y - p2.y)).abs(),
    }
}

/// Compute perimeter of shape
pub fn perimeter(shape: Shape) -> f32 {
    match shape {
        Shape::Point(_) => 0.0,
        Shape::Circle(Circle { center, radius }) => 2.0 * std::f32::consts::PI * radius,
        Shape::Triangle(Triangle { p1, p2, p3 }) => {
            let x1 = p1.x - p2.x;
            let y1 = p1.y - p2.y;
            let x2 = p2.x - p3.x;
            let y2 = p2.y - p3.y;
            let x3 = p3.x - p1.x;
            let y3 = p3.y - p1.y;
            (x1 * x1 + y1 * y1).sqrt() + (x2 * x2 + y2 * y2).sqrt() + (x3 * x3 + y3 * y3).sqrt()
        }
        Shape::Rectangle(Rectangle { p1, p2 }) => 2.0 * ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()),
    }
}
