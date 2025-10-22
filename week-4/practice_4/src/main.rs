//rust program to determine age pass

use std::io;
fn main() {

    let mut name = String::new();
    let mut age = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Not a valid string");

    println!("Enter your age: ");
    io::stdin().read_line(&mut age).expect("Not a avlid string");
    let age:i32 = age.trim().parse().expect("Not a valid number");

    if age >= 18{
        println!("Welcome to the party {}", name);
    } else {
        println!("Oops, youre not of age to enter the party {}", name);
    }

}