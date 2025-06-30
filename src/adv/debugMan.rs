use std::fmt::{Debug,Formatter,write};

struct User {
    name : String,
    age : u32
} 


impl Debug for User {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f,"User name is {}",self.name)
    }
}


fn main () {
    let u = User {
        name : String::from("Aswin"),
        age : 18
    };

    println!("{:?}",u); //only the name cause we had defined manually like that
}