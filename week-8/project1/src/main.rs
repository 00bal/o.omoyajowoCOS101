use std::io;

fn main() {
    // Define the levels and their corresponding experience ranges
    // We're using vectors here as per the project requirements
    let levels: Vec<&str> = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES"
    ];

    // Experience ranges: (min, max) years. For SES, assuming 14+ years, capped at 100 for simplicity
    let ranges: Vec<(u32, u32)> = vec![
        (1, 2),
        (3, 5),
        (5, 8),
        (8, 10),
        (10, 13),
        (14, 100)
    ];

    // Categories
    let categories: Vec<&str> = vec![
        "Office Administrator",
        "Academic",
        "Lawyer",
        "Teacher"
    ];

    // Positions for each category, indexed to match levels
    let positions: Vec<Vec<&str>> = vec![
        // Office Administrator
        vec![
            "Intern",
            "Administrator",
            "Senior Administrator",
            "Office Manager",
            "Director",
            "CEO"
        ],
        // Academic (note: "-" for APS 1-2, we'll handle it as invalid)
        vec![
            "-",
            "Research Assistant",
            "PhD Candidate",
            "Post-Doc Researcher",
            "Senior Lecturer",
            "Dean"
        ],
        // Lawyer
        vec![
            "Paralegal",
            "Junior Associate",
            "Associate",
            "Senior Associate 1-2",
            "Senior Associate 3-4",
            "Partner"
        ],
        // Teacher
        vec![
            "Placement",
            "Classroom Teacher",
            "Snr Teacher",
            "Leading Teacher",
            "Deputy Principal",
            "Principal"
        ]
    ];

    // Get user input
    let stdin = io::stdin();
    let mut lines = stdin.lines();

    println!("Enter the category (e.g., Lawyer, Teacher, Office Administrator, Academic):");
    let category:String = lines.next().unwrap().unwrap().trim().to_string();

    println!("Enter the position (e.g., Associate):");
    let position:String = lines.next().unwrap().unwrap().trim().to_string();

    println!("Enter years of experience (e.g., 6):");
    let experience_str: String = lines.next().unwrap().unwrap().trim().to_string();
    let experience:u32 = experience_str.parse().expect("Please enter a valid number");

    // Find the category index
    let mut cat_index: Option<usize> = None;
    for (i, cat) in categories.iter().enumerate() {
        if cat.to_lowercase() == category.to_lowercase() {  // Case insensitive for usability
            cat_index = Some(i);
            break;
        }
    }

    if cat_index.is_none() {
        println!("Invalid category.");
        return;
    }

    let cat_index = cat_index.unwrap();

    // Find the position index in the category's positions
    let mut pos_index: Option<usize> = None;
    for (i, pos) in positions[cat_index].iter().enumerate() {
        if pos.to_lowercase() == position.to_lowercase() {  // Case insensitive
            if *pos == "-" {
                println!("No valid position for this in the table.");
                return;
            }
            pos_index = Some(i);
            break;
        }
    }

    if pos_index.is_none() {
        println!("Invalid position for this category.");
        return;
    }

    let pos_index = pos_index.unwrap();

    // Get the level and range
    let level = levels[pos_index];
    let (min_exp, max_exp) = ranges[pos_index];

    // Validate experience
    if experience >= min_exp && experience <= max_exp {
        println!("Valid: The staff holds position {} in {}.", level, category);
    } else {
        println!("Invalid: Experience does not match the required range ({}-{}) for {} in {}.", min_exp, max_exp, level, category);
    }
}