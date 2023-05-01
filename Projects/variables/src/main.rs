fn main() {
   // let x = 5; gives error

    let mut x = 5;

    println!("The value of x is : {x}");

    x = 6;
    println!("The value of x is : {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS} seconds in 3 hours.");

    // shadowing
    let x = 7;
    let x  = 7 + 1; // declaring new variable x with the same name as previous variable
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {x}");
    }
    println!("The value of x is: {x}");

    // another purpose 

    // let mut spaces = " "; gives error
    let spaces = "      ";
    let spaces = spaces.len();
    println!("Total spaces available are: {spaces}");
}