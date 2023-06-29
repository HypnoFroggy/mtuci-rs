/*
fn main() {
let mut x = 5;
println!("The value of x is: {}", x);
x = 6;
println!("The value of x is: {}", x);
}
*/
/*
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
*/

//fn main() {
//    let mut spaces = "   ";
//    let spaces = spaces.len();
//}

// TYPES

/*
fn main() {
    // addition
    let sum = 5 + 10;
    println!("{}",sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}",difference);
    // multiplication
    let product = 4 * 30;
    println!("{}",product);
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{}",quotient);
    println!("{}",truncated);
    // remainder
    let remainder = 43 % 5;
    println!("{}",remainder);
}
*/
/*
fn main() {
    let t = true;

    //let f: bool = false; // with explicit type annotation
    let f = false;
    println!("{}",t);
    println!("{}",f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; // чуча заставила перекодировать файл
    println!("{}",c);
    println!("{}",z);
    println!("{}",heart_eyed_cat);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    println!("The value of x.0 is: {five_hundred}");
    let six_point_four = x.1;
    println!("The value of x.1 is: {six_point_four}");
    let one = x.2;
    println!("The value of x.2 is: {one}");
}
*/
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}