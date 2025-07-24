use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 0..500 {
            println!("{}",i);
        }
    });

    for i in 0..500 {
        println!("{}",i);
    }
}