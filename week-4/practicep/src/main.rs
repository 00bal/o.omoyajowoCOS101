use std::io::{self, Write};

/// Calculates the annual incentive based on experience and age.
///
/// This function implements the specific business logic provided:
/// 1. Inexperienced: 100,000
/// 2. Experienced & Age >= 40: 1,560,000
/// 3. Experienced & Age 30-39: 1,480,000
/// 4. Experienced & Age < 28: 1,300,000
/// 5. Experienced & Age 28 or 29: 0 (as this case is ignored)
fn calculate_incentive(is_experienced: bool, age: u32) -> u32 {
    // We can use underscores in numbers to make them more readable
    
    // Rule 1: Handle inexperienced employees first
    if !is_experienced {
        return 100_000;
    }

    // At this point, we know the employee IS experienced.
    // Now we just check the age ranges.
    
    if age >= 40 {
        // Rule 2
        1_560_000
    } else if age >= 30 {
        // Rule 3: This catches ages 30-39
        // (We don't need to check `age < 40` because `age >= 40` was already checked)
        1_480_000
    } else if age < 28 {
        // Rule 4
        1_300_000
    } else {
        // Rule 5: This is the gap (ages 28 and 29), which we agreed to ignore.
        // We return 0 as no incentive is defined.
        0
    }
}

/// Prompts the user until they enter a valid 'yes' or 'no'.
/// Returns `true` for "yes" and `false` for "no".
fn get_experience() -> bool {
    loop { // Loop until we get a valid answer
        print!("Is the employee experienced? (yes/no): ");
        
        // We must "flush" stdout to make sure the `print!` shows up
        // before the `read_line` call.
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // `.trim()` removes whitespace (like the newline)
        // `.to_lowercase()` makes the check case-insensitive
        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please type 'yes' or 'no'.");
                // The loop continues
            }
        }
    }
}

/// Prompts the user until they enter a valid positive number for age.
/// Returns the age as a `u32`.
fn get_age() -> u32 {
    loop { // Loop until we get a valid number
        print!("Please enter the employee's age: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // We try to `parse` the trimmed string into a u32 (unsigned 32-bit integer).
        // `match` is a clean way to handle the Result (Ok or Err) from `parse`.
        match input.trim().parse() {
            Ok(num) => return num, // Success! Return the number.
            Err(_) => {
                println!("Invalid input. Please enter a valid number (e.g., 35).");
                // The loop continues
            }
        }
    }
}

/// Main function to run the incentive calculator program.
fn main() {
    println!("--- Annual Incentive Calculator ---");

    // 1. Get inputs from the user
    let is_experienced = get_experience();
    let age = get_age();

    // 2. Calculate the incentive
    let incentive = calculate_incentive(is_experienced, age);

    // 3. Print the final result
    println!("-------------------------------------");
    // We use a comma to print the number with thousands separators
    println!("The employee's annual incentive is: N{}", incentive);
}