use std::io;

fn trapezium() {

    let mut height = String::new();
    println!("Enter the height of the trapezium:");
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height:f32 = height.trim().parse().expect("Please type a number");

    let mut base1 = String::new();
    println!("Enter the base of the trapezium:");
    io::stdin().read_line(&mut base1).expect("Failed to read line");
    let base1:f32 = base1.trim().parse().expect("Please type a number");

    let mut base2 = String::new();
    println!("Enter the base 2 of the trapezium:");
    io::stdin().read_line(&mut base2).expect("Failed to read line");
    let base2:f32 = base2.trim().parse().expect("Please type a number");

    let area_of_t:f32 = (height / 2.0) * (base1 * base2);

    println!("The area of the Trapezium is: {}", area_of_t);

}

fn rhombus() {

    let mut diagonal1 = String::new();
    println!("Enter the first side of the rhombus:");
    io::stdin().read_line(&mut diagonal1).expect("Failed to read line");
    let diagonal1:f32 = diagonal1.trim().parse().expect("Please type a number");

    let mut diagonal2 = String::new();
    println!("Enter the second side of the rhombus:");
    io::stdin().read_line(&mut diagonal2).expect("Failed to read line");
    let diagonal2:f32 = diagonal2.trim().parse().expect("Please type a number");

    let area_of_r:f32 = 0.5 * diagonal1 * diagonal2;

    println!("The area of the rhombus is: {}", area_of_r);
}

fn parallelogram(){

    let mut base = String::new();
    println!("Enter the base of the parallelogram:");
    io::stdin().read_line(&mut base).expect("Failed to read line");
    let base:f32 = base.trim().parse().expect("Please type a number");

    let mut altitude = String::new();
    println!("Enter the altitude of the parallelogram:");
    io::stdin().read_line(&mut altitude).expect("Failed to read line");
    let altitude:f32 = altitude.trim().parse().expect("Please type a number");

    let area_of_p:f32 = base * altitude;

    println!("The area of the parallelogram is: {}", area_of_p);
}

fn cube(){

    let mut length_of_side = String::new();
    println!("Enter the length of the cube of side:");
    io::stdin().read_line(&mut length_of_side).expect("Failed to read line");
    let length_of_side:f32 = length_of_side.trim().parse().expect("Please type a number");

    let area_of_cube:f32 = 6.0 * (length_of_side.powf(2.0));

    println!("The area of the cube of cube is: {}", area_of_cube);
}

fn cylinder(){

    let pi:f32 = 22.0 / 7.0;

    let mut radius = String::new();
    println!("Enter the radius of the cylinder:");
    io::stdin().read_line(&mut radius).expect("Failed to read line");
    let radius:f32 = radius.trim().parse().expect("Please type a number");

    let mut height = String::new();
    println!("Enter the height of the cylinder:");
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height:f32 = height.trim().parse().expect("Please type a number");

    let area_of_cylinder:f32 = pi * (radius.powf(2.0)) * height;

    println!("The volume of the cylinder is: {}", area_of_cylinder);

}

fn main() {

    let mut shape = String::new();
    println!("Pick the shape you want to find the area of \nTrapezium \nRhombus \nParallelogram \nCube \nCylinder");
    println!("--------------------------------------------------");
    io::stdin().read_line(&mut shape).expect("Failed to read line");
    let shape = shape.trim();

    if shape == "Trapezium" || shape == "trapezium" {
        trapezium();
    }

    else if shape == "Rhombus" || shape == "rhombus" {
        rhombus();
    }

    else if shape == "Parallelogram" || shape == "parallelogram" {
        parallelogram();
    }

    else if shape == "Cube" || shape == "cube" {
        cube();
    }

    else if shape == "Cylinder" || shape == "cylinder" {
        cylinder();
    }
}