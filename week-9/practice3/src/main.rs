use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Could not remove file");
    println!("File is removed");
}