use std::collections::{HashMap};

fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("Aswin"),19);
    users.insert(String::from("Suriya"),21);
    
    let aswin_age = users.get("Aswin");
    
    match aswin_age {
        Some(res) => {
            println!("{}",res);
        },
        None => {
            println!("{}","Not present");
        }
    }
}

// fn convert_to_hashmap(v : Vec<(String,u32)>) -> HashMap<String,u32> {
//     let mut hashMap = HashMap::new();
//     for (key,val) in v {
//         hashMap.insert(key,val);
//     }
//     return hashMap;
// }

// fn main() {
//     let users = vec![(String::from("Aswin"),18),(String::from("Suriya"),21)];
//     let hm = convert_to_hashmap(users);
//     println!("{:?}",hm);
// }
