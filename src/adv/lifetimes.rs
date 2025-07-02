//issue 
// fn main() {
//     let str1 = String::from("hello");
//     let res;
//     {
//         let str2 = String::from("world");
//         res = greatest_str(&str1,&str2);
//     }
//     //Dangling pointer error if str2 > str1
//     println!("{}",res);
// }

// fn greatest_str(s1:&String , s2:&String) -> &String {
//     if s1.len() > s2.len() {
//         s1
//     }
//     else {
//         s2
//     }
// }

//----
// fn main () {
//     let str1 = String::from("HEllo");
//     let res;
//     {
//         let str2 = String::from("Worlddd");
//         res = greatest(&str1,&str2);
//     }
//     println!("{}",res);
// }

// fn greatest<'a,'b>(s1 : &'a String , s2:&'b String) -> &'a String {
//     // here we are telling that the lifetime of s1 is 'a and the lifetime of s2 is 'b and we are returning the reference which has the lifetime of 'a
//     //s2 -> error lifetime may not live enough and the error will be in the main like str2 doesn't live enough
//     s1

// }

//---------------
// solution
fn main() {
    let str1 = String::from("hello");
    let ans;
    {
        let str2 = String::from("worldddd");
        ans = greatest(&str1, &str2);
        println!("{}",ans);
    }
    // the ans scope is until the end of main but the ans lifeime is only until the end of the scope ,of str2 ,so we can use ans only *until str2 lives
    // println!("{}",ans); error 
}

fn greatest<'a>(s1:&'a String,s2:&'a String) -> &'a String {
    // here we are telling that the two str is of lifetime 'a so the rust assumes the *smallest lifetime of both, in our case s2 and the ans is only valid until that point ie until s2
    if s1.len() > s2.len() {
        s1
    }
    else {
        s2
    }
}

