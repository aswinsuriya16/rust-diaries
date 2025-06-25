// Scalar types: integers, floats, booleans, chars
// Compound types: tuples and arrays
fn main() {
    //Integers
    let a = -128;  
    let b  = 1000;     
    println!("{} {}", a, b);

    //Floats 
    let pi = 3.14;
    let e = 2.71f32;         
    println!("{} {}", pi, e);

    // Booleans 
    let is_active = true;
    println!("Boolean is_active: {}", is_active);

    // Characters
    let letter = 'A';
    println!("Char letter: {}", letter);

    // Tuples 
    let person = ("Aswin", 18, true);
    let (name, age, employed) = person;
    println!("Tuple: {:?} Name: {} Age: {} Employed: {}", person, name, age, employed);

    // Arrays 
    let scores = [85, 90, 95];
    println!("Scores: {:?}", scores);
    println!("First score: {}", scores[0]);
    println!("Array length: {}", scores.len());

    // Slicing
    let slice = &scores[1..];
    println!("Slice of scores: {:?}", slice);

    // Type conversion 
    let num = 42;
    let num_str = num.to_string();
    let parsed: i32 = "100".parse().unwrap();
    println!("Converted: '{}', Parsed: {}", num_str, parsed);

    //vectors
    let mut xs = vec![1, 2, 3];
    print!("{}", xs.len());
    xs.push(4);
    print!("{}", xs.len());
}
