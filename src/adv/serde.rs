use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct User {
    name : String,
    password : String
}

// for display or just use the debug macro
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"The username is {} and the password is {}",self.name,self.password)
    }
}

fn main() {
    let u = User {
        name : String::from("Aswin"),
        password : String::from("ashhh")
    };

    let string_ver = serde_json::to_string(&u);
    // or use unwrap
    // let str_ver = string_ver.unwrap() -> it panicks if there is a fault

    match string_ver {
        Ok(res)=>{
            println!("Serialized string {}",res);
            let deser_ver = serde_json::from_str::<User>(&res); //turbofish syntax '::<User>' telling rust that the res is of User T type
            // or let deser_ver : User = serde_json::from_str(&res).unwrap(); 
            //or let deser_ver : Result<User,_> = serde_json::from_str(&res); 
            match deser_ver {
                Ok(deser_user)=>println!("{}",deser_user),
                Err(_)=>println!("{}","Error while deserializing")
            }
        }
        Err(_)=>println!("Error while converting")
    }

}