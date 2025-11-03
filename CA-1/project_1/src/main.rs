// Rust program to Evaluate grades of studen

use std::io;

fn main() {
    println!("\nStudent Grade Evaluator!");

    // input name
    println!("\nPlease Enter your name.");
    let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Your name is: {}", name);

    // input grade
    println!("\nEnter your grade.");
    let mut grade = String::new();
        io::stdin().read_line(&mut grade).expect("Failed to read input");
    let grade:f32 = grade.trim().parse().expect("Input not an integer");

     if grade >= 0.0 && grade <= 44.0 
    {
        println!("Your grade is an F");
    }
    else if grade > 45.0 && grade <= 49.0 
    {
        println!("Your grade is a D");
    }
    else if grade > 50.0 && grade <=59.0
    {
        println!("Your grade is a C");
    }  
    
    else if grade > 60.0 && grade <=69.50
    {
        println!("Your grade is a B");
    }
    else if grade > 70.0 && grade <=100.50
    {
        println!("Your grade is an A")
    }   

}
   