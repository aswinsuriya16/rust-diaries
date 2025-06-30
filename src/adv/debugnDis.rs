use std::fmt::Display;

#[derive(Debug)]
struct User {   
    username : String,
    age : u32
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //we are not returning a string — we are writing the string into the Formatter, which handles the output stream.
        write!(f,"The usernmae is {} and the age is {}",self.username,self.age)
    }
}

// impl Debug for User { this is the Debug trait 
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"The usernmae is {} and the age is {}",self.username,self.age)
//     }
// }

fn main() {
    let u = User {
        username : String::from("Aswin"),
        age :  18
    };
    //display
    println!("{}",u);
    //debug
    println!("{:?}",u);
}

