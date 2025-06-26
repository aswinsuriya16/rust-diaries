enum Shape {
    Circle(f32),
    Rectangle(f32,f32)
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Circle(rad)=> 3.14*rad*rad,
            Shape::Rectangle(l,b )=>l*b
        }
    }
}

fn main() {
    let c = Shape::Circle(2.0);
    let r = Shape::Rectangle(12.0, 12.0);
    println!("{}",c.area());
    println!("{}",r.area());
}