// A simple Rust program to find the developer with the most years of programming experience.
// We're using a struct to represent each candidate, and a vector to hold them all.

use std::io;

struct Candidate {
    name: String,
    years: u32,  // Years of experience, assuming non-negative integer.
}

fn main() {
    // Start with an empty list of candidates.
    let mut candidates = Vec::new();
    
    // Interactive loop to add candidates.
    loop {
        let mut input = String::new();
        println!("Enter candidate name (or 'done' to finish):");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let name = input.trim().to_string();
        if name.to_lowercase() == "done" {  // Made it case-insensitive for usability.
            break;
        }
        println!("Enter years of experience for {}:", name);
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let years: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number. Skipping this candidate.");
                continue;
            }
        };
        candidates.push(Candidate { name, years });
    }
    
    // Now, find the one with the highest years.
    if candidates.is_empty() {
        println!("No candidates to evaluate!");
        return;
    }
    
    let mut max_candidate = &candidates[0];
    for candidate in &candidates[1..] {
        if candidate.years > max_candidate.years {
            max_candidate = candidate;
        }
    }
    
    // Output the result.
    println!("The developer with the highest years of experience is:");
    println!("Name: {}", max_candidate.name);
    println!("Years: {}", max_candidate.years);
}