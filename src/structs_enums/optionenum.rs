//option enum
fn main() {
    let str = String::from("Hello World!");
    match find_e(&str) {
        Some(i)=>println!("Index found at {}",i),
        None=>println!("e not found")
    }
}

fn find_e(str : &String)->Option<u32> {
    let mut index = 0;
    for c in str.chars() {
        if c == 'e' {
            return Some(index);
        }
        index+=1;
    }
    None
}

