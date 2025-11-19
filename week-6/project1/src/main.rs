use std::io;

fn main() {
    
    println!("-------------------------------------");
    println!("                Menu                 ");
    println!("-------------------------------------");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken      - N3,000");
    println!("A = Amala & Ewedu Soup        - N2,500");
    println!("E = Eba & Egusi Soup          - N2,000");
    println!("W = White Rice & Stew         - N2,500");
    println!("-------------------------------------");

    println!("\nEnter the food code (P, F, A, E, W):");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read input");

    let p = 3200.0;
    let f = 3000.0;
    let a = 2500.0;
    let e = 2000.0;
    let w = 2500.0;
    

    println!("Enter the quantity:");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
    let quantity:f32 = quantity_input.trim().parse().expect("invalid amount");

    let food_code = food_code.trim();
    
    let mut price:f32 = 0.0;
    let mut food_name = ""; 

    if food_code == "P" || food_code == "p" {
        price = p;
        food_name = "Poundo Yam/Edinkaiko Soup";
    } 

    else if food_code == "F" || food_code == "f" {
        price = f;
        food_name = "Fried Rice & Chicken";
    } 

    else if food_code == "A" || food_code == "a" {
        price = a;
        food_name = "Amala & Ewedu Soup";
    } 

    else if food_code == "E" || food_code == "e" {
        price = e;
        food_name = "Eba & Egusi Soup";
    } 

    else if food_code == "W" || food_code == "w" {
        price = w;
        food_name = "White Rice & Stew";
    } 

    else {
        println!("Incorrect food code. Please restart.");
        return; 
    }

    let intial_P = price * quantity;
    let mut discount:f32 = 0.0;

    if intial_P > 10000.0 {
        discount = intial_P * 0.05;
    }

    let final_P = intial_P - discount;

    println!("\n|Summary of order|");
    println!("Food:       {}", food_name);
    println!("Quantity:   {}", quantity);
    println!("----------------------");
    println!("Total:      N{}", intial_P);
    println!("Discount:   N{}", discount);
    println!("Final Price: N{}", final_P);

}