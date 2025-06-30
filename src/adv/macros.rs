//declarative macros
// macro_rules! sayHello {
//     () => {
//         println!("Hello"); // to see how acually the println works internally ter-> cargo expand
//     };
// }

// fn main() {
//     sayHello!();
// }



macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name));
        }
    };
}

create_function!(hello);  // create a function called "hello"

fn main() {
    hello();  // Prints "Hello from hello"
}