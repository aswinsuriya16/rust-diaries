// package to work with time and dates
// use chrono::{Utc,Local};
// fn main() {
//     let utc_time = Utc::now();
//     let local_time = Local::now();
//     println!("{}",utc_time);
//     println!("{}",local_time);
// }



use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let url = env::var("SOME_ADDRESS");
    match url {
        Ok(str)=>println!("{}",str),
        Err(e)=>println!("Error while reading the addresss {}",e),
    }
}