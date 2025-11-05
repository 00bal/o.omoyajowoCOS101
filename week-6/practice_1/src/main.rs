fn main() {
    let name = "Obaloluwa Omoyajowo";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name {}", name);
    println!("University: {}, \n Address : {}",uni, addr );

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department, school );
}
