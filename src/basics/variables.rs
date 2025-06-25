// Variables are immutable by default
// Use mut keyword for mutable variables
// Shadowing allows redeclaring variables with same name
// Constants are always immutable and must have type annotations

fn main() {
    let x = 5;
    println!("Immutable variable x: {}", x);

    // Mutable variables
    let mut y = 10;
    println!("Mutable variable y before change: {}", y);
    y = 15;
    println!("Mutable variable y after change: {}", y);
    
    // Variable shadowing 
    let z = 20;
    println!("First z: {}", z);
    
    let z = z + 5; // Shadowing with transformation
    println!("Shadowed z: {}", z);
    
    let z = "Now im a string"; // Can even change type
    println!("Shadowed z with different type: {}", z);
    
    // Constants are always immutable and must have type annotation
    const MAX_POINTS: u32 = 100_000;
    println!("Constant MAX_POINTS: {}", MAX_POINTS);
    
    // Destructuring
    let (a, b) = (1, 2);
    println!("Destructured values - a: {}, b: {}", a, b);
    
    // Unused variables starts with _ to avoid warnings
    let _unused = "I won't be used";
}