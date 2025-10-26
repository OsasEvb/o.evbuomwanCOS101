use std::io;  // For reading input from keyboard

fn main() {
    println!("Enter the value of a:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Please enter a number");

    println!("Enter the value of b:");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Please enter a number");

    println!("Enter the value of c:");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Please enter a number");

    // Calculate the discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Determine the nature of roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {:.2}", root);
    } else {
        println!("No real roots (discriminant is negative)");
    }
}
