fn main() {
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	let a = p*(1.0 + r / 100.0).powf(n);
	let ci = a-p;

	println!("Amount is: ₦{:.2}", a);
	println!("Compound Interest is: ₦{:.2}", ci);
	
}