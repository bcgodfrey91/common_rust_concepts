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

    // floating points
    let z = 2.02; // f64

    let zz: f32 = 3.01; // f32

    println!("The value of z is: {}", z);
    println!("The value of zz is: {}", zz);

    // numeric opperations
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);

    // character type
    let c = "z";
    let cat = "😻";

    println!("The value of c is: {}", c);
    println!("The value of cat is: {}", cat);

    // tuple
    let tup = (500, "hello", 1);

    let (a, b, c) = tup;

    println!("The value of b is: {}", b);
    println!("The value of a is: {}", a);

    let nums: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = nums.0;

    let six_point_four = nums.1;

    let one = nums.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // arrays
    // let arr = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = arr[index];

    // println!("The value of element is: {}", element);

    another_function();
}


fn another_function() {
    println!("Another function.");
}
