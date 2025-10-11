fn main() {
	let p:f64 = 510_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;
	
	let a = p * (1.0 + r / 100.0).powf(n);
	let ci = a - p;

	println!("Depreciation is: ₦{:.2}", ci);
	println!("Value after 3 years: ₦{:.2}", p - ci);
	
}