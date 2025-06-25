// fn main() {
//     let x = 99;
//     let is_even = is_even(x);
//     if is_even {
//         print!("{} is even", x);
//     } else {
//         print!("{} is odd", x);
//     }
// }

// fn is_even(x: i32) -> bool {
//     return x % 2 == 0;
// }

pub fn main() {
    let str = String::from("Aswin Suriya");
    println!("First name {}", get_first_name(str))
    
}

pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;
}