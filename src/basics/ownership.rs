fn main() {
    problem();
    clone_fix();
    return_ownership();
}

fn problem() {
    //let s1 = String::from("hello");
    //let s2 = s1;
    //println!("{}", s1); // This line would cause a compile error because ownership has been transferred.
}

fn clone_fix() {
    let str = String::from("Hi there!");
    let str1 = str.clone();
    print!("{} {}",str,str1);
}

fn return_ownership() {
    let mut str = String::from("Hello");
    str = temporary_pass(str);
    print!("{}",str);

    
}

fn temporary_pass(temp_str : String)->String {
    //now the str in this function is the owner
    print!("{}",temp_str);
    // now we had passed the ownership back to the original 
    temp_str
}
