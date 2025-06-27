use std::f32::consts::PI;

trait Shape {
    fn area(&self) -> f32;
}
struct Rectangle {
    length : f32,
    width : f32
}

struct Circle {
    radius : f32
}


impl Shape for Rectangle {
    fn area(&self) -> f32 {
        return self.length*self.width;
    }
}


impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI*self.radius*self.radius;
    }
}

fn print_area<T:Shape>(s:T) {
    println!("{}",s.area());
}


fn main() {
    let r = Rectangle {
        length : 12.0,
        width : 11.0
    };
    let c = Circle {
        radius : 8.0,
    };

    print_area(r);
    print_area(c);
}