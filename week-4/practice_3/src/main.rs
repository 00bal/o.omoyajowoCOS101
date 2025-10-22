//rust program to calculate the area of a triangle given base and height

use std::io;

fn main(){
    let mut base = String::new();
    let mut height = String::new();

    println!("Enter base: ");
    io::stdin().read_line(&mut base).expect("Not a valid string");
    let base:f32 = base.trim().parse().expect("Not a valid number");

    println!("Enter height: ");
    io::stdin().read_line(&mut height).expect("Not a valid string");
    let height:f32 = height.trim().parse().expect("Not a valid number");

    if base > 0.0 {
        let area:f32 =(base * height) / 2.0;
        println!("Area of a triangle: {}", area);

    }
}
