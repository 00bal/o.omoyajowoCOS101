fn main() {
    
    //using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of the vectore
    println!("\n The length of the Vec::new is {}", v.len());

    //usning macro
    let v = vec!["Grace", "Effoing", "Basil","Kareem", "Susan"];
    println!("\n The size of the vec! is: {}", v.len());
}