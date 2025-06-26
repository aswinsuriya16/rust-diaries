// fn main() {
//     let name = String::from("Aswin Suriya");
//     // passing the name as the reference, ownership remains to name
//     get_length(&name);
// }

// fn get_length(name :&String) -> usize {
//     return name.len();
// }

// fn main() {
//     //we can have the multiple immutable refernces 
//     let str = String::from("Hi there");
//     let s1 = &str;
//     let s2 = &str;
//     print!("{} {} {}",s1,s2,str);
// }

fn main() {
    // we can have the single mutable reference but no other immutable references

    let mut str = String::from("Hi there");
    let s1 = &mut str;
    //let s2 = &mut str; this line would cause the compilation error as s1 is still in use and we cant have the multiple mut references 
    //let s3 = &str; same for this 
    print!("{}",s1)
}

