struct Person {
    name : String,
    age : i32,
}


//Struct Implementation
impl Person {
    fn new(name:String,age:i32)->Self {
        Self{name,age}
    }

    fn is_major(&self)-> String {
        if self.age >= 18 {
            return String::from("Is major");
        }
        else{
            return String::from("Not a Major");
        }
    }

    fn staticFn()->String{
        String::from("This is the static function")
    }
}

fn main () {
    let p1 = Person {
        name : String::from("Aswin"),
        age : 18,
    };

    println!("{}",p1.name);
    println!("{}",p1.is_major());
    println!("{}",Person::staticFn());
}