//Rust program to read the height of a person
// and then print if the person is tall, a dwarf
//or average height

use std::io;

fn main(){
    let mut height = String::new();

    println!("Enter your height (in centimeters): ");
    io::stdin().read_line(&mut height).expect("Not a valid string");
    let height:f32 = height.trim().parse().expect("Not an valid number");

    if height >= 150.0 && height <= 170.0{
        println!("You are am average height person");
    }
    else if height > 170.0 && height <= 195.0{
        println!("You are tall");
    }
    else{
        println!("ABnormal height");
    }
}