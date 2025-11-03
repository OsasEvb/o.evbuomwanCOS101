// We use the "io" library to read input from the keyboard
use std::io;

fn main() {
    // Ask the user for 'a' and read it
    println!("Enter the value of a:");
    let mut a = String::new(); // make an empty string to store user input
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Please enter a number"); // convert text to number

    // Ask for 'b'
    println!("Enter the value of b:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Please enter a number");

    // Ask for 'c'
    println!("Enter the value of c:");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Please enter a number");

    // Calculate discriminant (b^2 - 4ac)
    let discriminant = b * b - 4.0 * a * c;

    // Check if discriminant is positive, zero, or negative
    if discriminant > 0.0 {
        // Two distinct real roots
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);

    } else if discriminant == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        println!("One real root: {:.2}", root);

    } else {
        // Negative discriminant → No real roots
        println!("No real roots (discriminant is negative)");
    }
}