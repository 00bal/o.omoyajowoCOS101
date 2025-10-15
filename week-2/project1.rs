fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	// Amount
	let a = p[1.0 + (r / 100)] ^ n
	println!("Amount is {}", a);
	let cl = a - p
	println!("compound interest is {}", cl);
	
}