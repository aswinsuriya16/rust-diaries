use std::f32::consts::PI;
trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T> {
    length: T,
    width: T,
}

struct Circle {
    radius: f32,
}

impl<T: std::ops::Mul<Output = T> + Copy> Shape<T> for Rectangle<T> {
    fn area(&self) -> T {
        self.length * self.width
    }
}

impl Shape<f32> for Circle {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}

fn print_area<T,U>(s:&T)
where 
    T : Shape<U>,
    U : std::fmt::Display {
        println!("{}",s.area());
    } 


//by debug
fn print_area_debug<T,U>(s:&T)
where 
    T : Shape<U>,
    U : std::fmt::Debug {
        println!("{:?}",s.area())
    }

    
fn main() {
    let c = Circle { radius: 12.2 };
    println!("Circle : {}", c.area());

    let r = Rectangle { length: 10, width: 5 };
    println!("Rectangle : {}", r.area());
    print_area(&c);
    print_area(&r);
    print_area_debug(&r);
}
