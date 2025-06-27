fn main() {
    print_fn(String::from("Hello"));
    print_fn(12);
    print_fn(12.21);

    let res = find_biggest(121,213);
    println!("The biggest is {}",res);
}

// ord trait is already in scope   ,prelude
fn find_biggest<T : Ord >(a:T , b:T) -> T {
    if a > b {
        return a;
    }
    b
}

fn print_fn<T:std::fmt::Display>(a : T) {
    println!("{}",a);
}


