fn main() {
    // loop {
    //     println!("deliberately");
    // }

    // Returning Values from Loops
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 11;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    // Conditional loops with while
    let mut number = 4;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("FINISHED!!");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
