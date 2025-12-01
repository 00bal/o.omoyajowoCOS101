fn main() {

    // List of commissioners
    let commissioners = vec![
        "Aigboagun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogobona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    // List of ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // List of geopolitical zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Print merged output
    println!("MERGED EFCC DATA\n");

    for i in 0..commissioners.len() {
        println!("Record {}", i + 1);
        println!("Commissioner: {}", commissioners[i]);
        println!("Ministry: {}", ministries[i]);
        println!("Geopolitical Zone: {}", zones[i]);
        println!("--------------------------------------");
    }
}
