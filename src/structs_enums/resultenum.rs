use std::fs;
// result enum
fn main() {
    let f = fs::read_to_string("sample.txt");
    match f {
        Ok(contents)=>println!("{}",contents),
        Err(e)=>println!("{}",e)
    }
}