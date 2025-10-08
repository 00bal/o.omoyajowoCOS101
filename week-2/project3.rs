fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// Amount
	let amt = p[1.0 - (r / 100)] ^ n
	println!("The reduced value of the tv is {}", amt);
	
}