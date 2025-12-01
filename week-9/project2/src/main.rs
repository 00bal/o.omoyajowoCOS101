use std::fs::File;
use std::io::{Write, Result};

struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() -> Result<()> {
    // 1. Create an array of students (hard-coded for simplicity)
    let students = [
        Student {
            name: "Oluchi Mordi".to_string(),
            matric: "ACC1021111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 200,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric: "CSC10328828".to_string(),
            department: "Computer Science".to_string(),
            level: 300,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 300,
        },
    ];

    // 2. Display the records on the screen
    println!("PAU SMIS");
    println!("---------------------------------------------------------------");
    println!(
        "{:<20} {:<15} {:<20} {}",
        "Student Name", "Matric No", "Department", "Level"
    );
    println!("---------------------------------------------------------------");

    for s in &students {
        println!(
            "{:<20} {:<15} {:<20} {}",
            s.name, s.matric, s.department, s.level
        );
    }

    // 3. Save to a CSV file
    let mut file = File::create("students.csv")?;

    writeln!(file, "Student Name,Matric Number,Department,Level")?;
    for s in &students {
        writeln!(file, "{},{},{},{}", s.name, s.matric, s.department, s.level)?;
    }

    println!("\nData saved to students.csv successfully!");

    Ok(())
}
