fn main() {
    println!("Hello, world!");

    for n in 0..15 {
        println!("Fibo({}) = {}",n,fibo(n));
    }
}


fn fibo(num:u32) -> u32 {
    if num <= 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } 

    fibo(num - 1) + (num - 2)   
}