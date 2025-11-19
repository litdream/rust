use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

// Circle
pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// Square
pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}


fn main() {

    let circle = Circle { radius: 7.0 };
    let square = Square { side: 5.0 };

    println!("Circle (Radius 7.0) Area: {:.2}", circle.area());
    println!("Square (Side 5.0) Area: {:.2}", square.area());

    // Using <Box<dyn Shape>> to match sizes.
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(circle),
        Box::new(square),
        Box::new(Circle { radius: 3.0 }),
    ];


    // Polymorphysm
    let mut total_area = 0.0;
    for (i, shape) in shapes.iter().enumerate() {
        let area = shape.area();
        println!("Shape {}: Area = {:.2}", i + 1, area);
        total_area += area;
    }
    println!("Total Area of all shapes: {:.2}", total_area);
}
