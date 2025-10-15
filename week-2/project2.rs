fn main () {
    let tosh:f64 = 450000.0;
    let mac:f64 = 1500000.0;
    let hp:f64 = 750000.0;
    let dell:f64 = 2850000.0;
    let acer:f64 = 2500000;

    // sum
    let sum = (tosh * 2.0) + mac + (hp * 3) + (dell * 3) + acer
    println!("sum of all product sales is {}", sum);

    //average
    let avg = sum/10
    println!("The averege of all the product sales is {}", avg);
    
}