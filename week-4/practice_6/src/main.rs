//Rust program to count numbers

use std::io;

fn main(){
    println!("Enter lower boundry");
    let mut lower_boundry = String::new();
    io::stdin().read_line(&mut lower_boundry).expect("Failed to read input");
    let lower_boundry = lower_boundry.trim().parse().expect("Failed to input");

    println!("Enter Upper boundry");
    let mut upper_boundry = String::new();
    io::stdin().read_line(&mut upper_boundry).expect("Failed to read input");
    let upper_boundry:i32 = upper_boundry.trim().parse().expect("Failed to input");

    for x in lower_boundry..upper_boundry{//upper boundry is not inclusive
        println!("Count level is {} ", x);
    }

}