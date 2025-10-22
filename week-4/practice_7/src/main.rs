use std::io;

fn main(){

    println!("Enter a number");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let mut num:i32 = num.trim().parse().expect("Failed to input");

    while num < 10{
        println!("Inside the loop is {}", num);
        num+=1
    }
    println!("outside loop is {}", num);
}