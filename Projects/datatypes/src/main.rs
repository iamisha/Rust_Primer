fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("The value of the x is {x}");

    println!("The value of the y is {y}");



    // Numeric Operations

    // addition
    let sum = 5 + 10;

    // substraction
    let difference = 95.5 - 4.3;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("{sum}");
    println!("{difference}");
    println!("{quotient}");
    println!("{truncated}");



    // Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("{t}");
    println!("{f}");


    // The Character Type

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let thumbs_up = '👍';
    println!("{thumbs_up}");


    // Tuple example 1
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{six_point_four}");
    
    // empty tuple  is called the unit
    
}