use::std::io::Write;
use::std::io::Read;

fn main() {

    //Creating the txt file
    let mut file = std::fs::File::create("Nigerian-Brewery-Limited.txt").expect("Create failed");
    
    //Writing the contents into the txt file
    file.write_all("Types of drinks sold: Lager, Stout, Non-Alcoholic\n"
        .as_bytes()).expect("Write failed");

    file.write_all("Lager drinks: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n"
        .as_bytes()).expect("Write failed");

    file.write_all("Stout drinks: Legend, Turbo king, Williams\n"
        .as_bytes()).expect("Write failed");

    file.write_all("Non-Alcoholic drinks: Maltina, Amste Malta, Malta Gold, Fayrouz\n"
        .as_bytes()).expect("Write failed");

    println!("\nData written to file");

    //Reading the txt file
    let mut file = std::fs::File::open("Nigerian-Brewery-Limited.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}