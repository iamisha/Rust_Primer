fn main () {
    let number = 8;

    // changing the value of number
    // let number = 7;
    // if number < 5 {
    //     println!("condition ture cha");
    // } else {
    //     println!("Condition false cha");
    // }

    // handling multiple conditions with else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3, and 4");

    }

    // Using if in a let statement
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is {number}");
}