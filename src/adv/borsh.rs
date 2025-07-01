use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize,BorshSerialize,Debug)]
struct User {
    name : String,
    password : String
}
fn main() {
    let u = User {
        name : String::from("Aswin"),
        password : String::from("ashh")
    };

    let mut vec: Vec<u8> = Vec::new();
    let _res = u.serialize(&mut vec);
    println!("{:?}",vec);
    let user = User::try_from_slice(&vec);
    match user {
        Ok(res)=>{
            println!("{:?}",res);
        },
        Err(_)=>println!("Error in deserialiazing")
    }
}