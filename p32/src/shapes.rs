pub trait Shape: std::fmt::Debug {
    const NAME: &'static str;

    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn scale(&mut self, factor: f32);

    fn area_to_perimeter(&self) -> f32 {
        self.area() / self.perimeter()
    }

    fn biggest_area<'a>(first: &'a Self, second: &'a Self) -> &'a Self {
        if first.area() > second.area() {
            first
        } else {
            second
        }
    }

    fn print_properties(&self) {
        println!(
            "Name: {0}, Area: {1}, Perimeter: {2}",
            Self::NAME,
            self.area(),
            self.perimeter()
        );
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

#[derive(Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

#[derive(Debug)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

#[derive(Debug)]
pub enum DynamicShape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

impl Shape for Point {
    const NAME: &'static str = "Point";

    fn area(&self) -> f32 {
        0.0
    }

    fn perimeter(&self) -> f32 {
        0.0
    }

    fn area_to_perimeter(&self) -> f32 {
        0.0
    }

    fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl Shape for Triangle {
    const NAME: &'static str = "Triangle";

    fn area(&self) -> f32 {
        let x1 = self.p1.x - self.p2.x;
        let y1 = self.p1.y - self.p2.y;
        let x2 = self.p2.x - self.p3.x;
        let y2 = self.p2.y - self.p3.y;
        let x3 = self.p3.x - self.p1.x;
        let y3 = self.p3.y - self.p1.y;
        (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs() / 2.0
    }

    fn perimeter(&self) -> f32 {
        let x1 = self.p1.x - self.p2.x;
        let y1 = self.p1.y - self.p2.y;
        let x2 = self.p2.x - self.p3.x;
        let y2 = self.p2.y - self.p3.y;
        let x3 = self.p3.x - self.p1.x;
        let y3 = self.p3.y - self.p1.y;
        (x1 * x1 + y1 * y1).sqrt() + (x2 * x2 + y2 * y2).sqrt() + (x3 * x3 + y3 * y3).sqrt()
    }

    fn scale(&mut self, factor: f32) {
        self.p1.scale(factor);
        self.p2.scale(factor);
        self.p3.scale(factor);
    }
}

impl Shape for Circle {
    const NAME: &'static str = "Circle";

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn scale(&mut self, factor: f32) {
        self.center.scale(factor);
        self.radius *= factor;
    }
}

impl Shape for Rectangle {
    const NAME: &'static str = "Rectangle";

    fn area(&self) -> f32 {
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }

    fn perimeter(&self) -> f32 {
        2.0 * ((self.p1.x - self.p2.x).abs() + (self.p1.y - self.p2.y).abs())
    }

    fn scale(&mut self, factor: f32) {
        self.p1.scale(factor);
        self.p2.scale(factor);
    }
}

impl Shape for DynamicShape {
    const NAME: &'static str = "DynamicShape";

    fn area(&self) -> f32 {
        match self {
            DynamicShape::Point(point) => point.area(),
            DynamicShape::Circle(circle) => circle.area(),
            DynamicShape::Triangle(triangle) => triangle.area(),
            DynamicShape::Rectangle(rectangle) => rectangle.area(),
        }
    }

    fn perimeter(&self) -> f32 {
        match self {
            DynamicShape::Point(point) => point.perimeter(),
            DynamicShape::Circle(circle) => circle.perimeter(),
            DynamicShape::Triangle(triangle) => triangle.perimeter(),
            DynamicShape::Rectangle(rectangle) => rectangle.perimeter(),
        }
    }

    fn scale(&mut self, factor: f32) {
        match self {
            DynamicShape::Point(point) => point.scale(factor),
            DynamicShape::Triangle(triangle) => triangle.scale(factor),
            DynamicShape::Circle(circle) => circle.scale(factor),
            DynamicShape::Rectangle(rectangle) => rectangle.scale(factor),
        };
    }
}

#[derive(Debug)]
pub enum Return<'a, 'b, T, U> {
    First(&'a T),
    Second(&'b U),
}

pub fn biggest_area_to_perimeter<'a, 'b, T: Shape, U: Shape>(
    first: &'a [T],
    second: &'b [U],
) -> Return<'a, 'b, T, U> {
    let firstoption = first
        .iter()
        .map(|shape| (shape, shape.area_to_perimeter()))
        .reduce(|(shape1, value1), (shape2, value2)| {
            if value1 > value2 {
                (shape1, value1)
            } else {
                (shape2, value2)
            }
        });
    let secondoption = second
        .iter()
        .map(|shape| (shape, shape.area_to_perimeter()))
        .reduce(|(shape1, acc), (shape2, value)| {
            if acc > value {
                (shape1, acc)
            } else {
                (shape2, value)
            }
        });

    let ret = match (firstoption, secondoption) {
        (Some((fmaxshape, _)), None) => Return::First(fmaxshape),
        (Some((fmaxshape, fmax)), Some((_, smax))) if fmax > smax => Return::First(fmaxshape),
        (_, Some((smaxshape, _))) => Return::Second(smaxshape),
        (None, None) => panic!(),
    };
    println!("{:#?}", ret);
    ret
}
