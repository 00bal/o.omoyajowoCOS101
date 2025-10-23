use std::io;

fn main(){

    let mut age = String::new();
    let mut exp = String::new();

    println!("Enter your age:");
    io::stdin().read_line(&mut age).expect("Couldnt read string");
    let age:i32 = age.trim().parse().expect("Couldnt determine age");

    println!("Are you experienced (yes/no)");
    io::stdin().read_line(&mut exp).expect("Couldnt read string");
    let exp = exp.trim().to_lowercase();

    println!("--------------------------------");

    if exp == "no" || exp == "n" {
        println!("Your incentive is N100,000");
    }

    else if age >=40 && exp == "yes" {
        println!("Your incentive is N1,560,000");
    }

    else if age >=30 && exp == "yes" {
        println!("Your incentive is N1,480,000");
    }

    else if age < 28 && exp == "yes" {
        println!("Your incentive is N1,300,000");

    }

    else {
        println!("There is no incentive for your group");
    }


}