fn main() {
	let tf: f64 = 2.0;
	let mf: f64 = 1.0;
	let hf: f64 = 3.0;
	let df: f64 = 3.0;
	let af: f64 = 1.0;

	let tx: f64 = 450_000.0;
	let mx: f64 = 1_500_000.0;
	let hx: f64 = 750_000.0;
	let dx: f64 = 2_850_000.0;
	let ax: f64 = 250_000.0;

	let sum_fx = tf*tx + mf*mx + hf*hx + df*dx + af*ax;

	let sum_f = tf + mf + hf + df + af;

	let mean = sum_fx/sum_f;
	println!("The average cost is: ₦{:.2}", mean);

}