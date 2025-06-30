#[derive(Clone, Copy,Debug)]
struct User {
    is_male : bool,// implents copy
    age : u32, // implements copy
    // so the struct also now implemets copy when we derive the macro
    //name: String if string is here cant implement copy
}

fn main() {
    let u1 = User{ 
        is_male : true,
        age : 18,
        //name : String::from("as")
    };

    let u2 = &u1;
    println!("{:?}",u2);
}