struct Rectangle<T> {
    length : T,
    width : T
}


// we do need a copy trait as the self itself is a borrow, in area case so we cant move the ownership to mul trait 
impl<T: std::ops::Mul<Output = T > + Copy> Rectangle<T> {
    fn area(&self)-> T {
        self.length*self.width
    }
}


fn main() {
    let r = Rectangle{
        length : 12.3,
        width :11.21
    };
    
    let r2 = Rectangle {
        length : 12,
        width : 11
    };
    println!("{}",r.area());
    println!("{}",r2.area());
}