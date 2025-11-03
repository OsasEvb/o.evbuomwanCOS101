// Rust program to calculate compound interest and total amount

fn main() {
    let p:f64 = 4000.0;
    
    let r:f64 = 2.0;
    
    let t:f64 = 3.0;

    // compound interest
    let a = p *((1.0 + r / 100.0))* t;
    
    println!(" Amount is {}", a);

    let ci = a - p;
    println!("Compound interest is {}", ci);


}