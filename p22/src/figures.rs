pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

/// Compute area of point
pub fn area_point(_: Point) -> f32 {
    0.0
}

/// Compute area of circle
pub fn area_circle(Circle { center: _, radius }: Circle) -> f32 {
    std::f32::consts::PI * radius * radius
}

/// Compute area of triangle
pub fn area_triangle(Triangle { p1, p2, p3 }: Triangle) -> f32 {
    (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y)).abs() / 2.0
}

/// Compute area of rectangle
pub fn area_rectangle(Rectangle { p1, p2 }: Rectangle) -> f32 {
    ((p1.x - p2.x) * (p1.y - p2.y)).abs()
}

/// Compute area of shape
pub fn area(shape: Shape) -> f32 {
    match shape {
        Shape::Point(point) => area_point(point),
        Shape::Circle(circle) => area_circle(circle),
        Shape::Triangle(triangle) => area_triangle(triangle),
        Shape::Rectangle(rectangle) => area_rectangle(rectangle),
    }
}

// Compute perimeter of point
pub fn perimeter_point(_: Point) -> f32 {
    0.0
}

/// Compute perimeter of circle
pub fn perimeter_circle(Circle { center: _, radius }: Circle) -> f32 {
    2.0 * std::f32::consts::PI * radius
}

/// Compute perimeter of triangle
pub fn perimeter_triangle(Triangle { p1, p2, p3 }: Triangle) -> f32 {
    let x1 = p1.x - p2.x;
    let y1 = p1.y - p2.y;
    let x2 = p2.x - p3.x;
    let y2 = p2.y - p3.y;
    let x3 = p3.x - p1.x;
    let y3 = p3.y - p1.y;
    (x1 * x1 + y1 * y1).sqrt() + (x2 * x2 + y2 * y2).sqrt() + (x3 * x3 + y3 * y3).sqrt()
}

/// Compute perimeter of rectangle
pub fn perimeter_rectangle(Rectangle { p1, p2 }: Rectangle) -> f32 {
    2.0 * ((p1.x - p2.x).abs() + (p1.y - p2.y).abs())
}

/// Compute perimeter of shape
pub fn perimeter(shape: Shape) -> f32 {
    match shape {
        Shape::Point(point) => perimeter_point(point),
        Shape::Circle(circle) => perimeter_circle(circle),
        Shape::Triangle(triangle) => perimeter_triangle(triangle),
        Shape::Rectangle(rectangle) => perimeter_rectangle(rectangle),
    }
}
