//anonymous functions
fn main() {
    let msg = String::from("Hi there!");
    let greet = || {
        println!("{}",msg);
    };
    greet();
}
