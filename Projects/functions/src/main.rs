// snake case 
fn main() {
    println!("Hello, world!");

   // another_function();

   // parameters
   another_function(5);

   // defining the multiple parameters
   another_function_2nd(5, 'm');
}
// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x: i32){
    println!("The value of x is: {x}");
}
fn another_function_2nd(value: i32, unit_label: char) {
    println!("The length is: {value}{unit_label}");
}