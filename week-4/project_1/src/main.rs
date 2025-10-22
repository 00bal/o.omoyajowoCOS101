//Rust program to calculate quadratic formula

use std::io;

fn main(){

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter a");
    io::stdin().read_line(&mut a).expect("problem with input");
    let a:f32 = a.trim().parse().expect("couldnt input");

    println!("Enter b");
    io::stdin().read_line(&mut b).expect("problem with input");
    let b:f32 = b.trim().parse().expect("couldnt input");

    println!("Enter c");
    io::stdin().read_line(&mut c).expect("problem with input");
    let c:f32 = c.trim().parse().expect("couldnt input");

    let q:f32 = b.powi(2) - (4.0 * a *c);

    if q > 0.0{
        let root1:f32 = (-b + q.sqrt()) / 2.0;
        let root2:f32 = (-b + q.sqrt()) / 2.0;

        println!("Roots (x): {} {}.",root1,root2 );
    }
    else if q == 0.0 {
        let root3:f32 = -b/2.0;
        println!("Single root {}", root3);
    }
    else{
        println!("No real roots");
    }
}