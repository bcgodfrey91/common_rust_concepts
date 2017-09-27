#![allow(unused_variables)]
fn main() {
    // variables and mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of x is: {}", y);

    let spaces = "1234";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    // data types
    
}
