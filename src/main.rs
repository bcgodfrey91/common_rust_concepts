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
}
