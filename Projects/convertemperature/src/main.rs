// imports the io module that provides
// input and output functionality to the program
use std::io;

// function that returns nothing
fn main() -> () {
    println!("Options::You want to covert to Farhenheit or Celsius::Enter 'c' for Celsius or 'f' for Farhenheit...");

    // ask the user which temperature scale user want to convert to and initialize to a new empty string
    let mut convert_type = String::new();

    // reads a line of text from standard input and stores in the 'convert_type' variable. if an error occurs while reading the input, 
    // will print "Faild to conversion type"
    io::stdin().read_line(&mut convert_type)
        .expect("Failed to conversion type");

    let t = String::from(convert_type);
    println!("You want to convert to: {}", t);
    println!("What temperature would you like to convert?");
    let mut temp = String::new();
    
    io::stdin().read_line(&mut temp)
        .expect("Failed to read temperature.");
    
    let temp: i32 = match temp.trim().parse() {
        Ok(temp) => temp,
        Err(_e) => {
            -1
        }
    };
    
    match t.as_str() {
        "C\n" => println!("{}", ftoc(temp)),
        "F\n" => println!("{}", ctof(temp)),
        _ => println!("t = {:?}", t),
    }
 }
    
    // Celsius to Fahrenheit
    fn ctof(c: i32) -> i32 {
        (c * (9 / 5)) + 32
    
    }
    
    //Fahrenheit to Celsius
    fn ftoc(f: i32) -> i32 {
        (f-32) * (5 / 9)
    }