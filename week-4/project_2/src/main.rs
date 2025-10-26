// Rust program for age and experience for empoyees

use std::io;  // for reading input from keyboard

fn main() {
    // Ask the user for experience
    println!("Is the employee experienced? (yes or no):");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase(); // convert to lowercase

    // Ask the user for age
    println!("Enter the age of the employee:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Please enter a valid number");

    // Decide incentive using if-else conditions
    if experience == "yes" {
        if age >= 40 {
            println!("The annual incentive is ₦1,560,000");
        } else if age >= 30 && age < 40 {
            println!("The annual incentive is ₦1,480,000");
        } else if age < 28 {
            println!("The annual incentive is ₦1,300,000");
        } else {
            println!("The annual incentive is ₦1,000,000 (default for experienced employee)");
        }
    } else {
        println!("The annual incentive is ₦100,000");
    }
}