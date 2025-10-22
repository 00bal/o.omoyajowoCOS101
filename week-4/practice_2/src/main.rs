// rust program to calculate the area of a triangle give threee sides

use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter first edge of triangle: ");
    io::stdin()
    .read_line(&mut a)
    .expect("Not a valid number");
    let a:f32 = a.trim().parse().expect("Not a valid number");

    println!("Enter the second edge of teh triangle: ");
    io::stdin()
    .read_line(&mut b)
    .expect("Not a valid number");
    let b:f32 = b.trim().parse().expect("Not a valid number");

    println!("Enter the third edge of the triangle: ");
    io::stdin().read_line(&mut c).expect("Not a valid number");
    let c:f32 = c.trim().parse().expect("Not a valid number");

    let s:f32 = (a + b + c) / 2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("Area of a triangle {}", area);
    
}