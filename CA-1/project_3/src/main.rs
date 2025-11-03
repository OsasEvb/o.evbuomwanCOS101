// Rust program to manage simple purchase orders for a computer store

use std::io;

fn main() {
    let mut l = 550.000;
    let mut m = 550.000;
    let mut k = 550.000;
    let mut h = 550.000;


    println!("\nSimple Inventory Discount System!");
    // input item code
    println!("\nPlease Enter Item code.");
    let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Item code is: {}", name);

    // input quantity
    println!("\nPlease Enter quantity.");
    let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Quantity is: {}", name);

     if price >= 25.000.0  &&  <=25.000.0;
       {  println!("Your item is a Headset");
    }
    else if price > 15.000.0 && grade <= 15.000.0 
    {
        println!("Your item is a Keyboard");
    }
    else if price  > 120.000.0 && grade <= 120.000.0
    {
        println!("Your item is a monitor");
    }  
    
    else if item  > 550.000.0 && grade <= 550.000.0
    {
        println!("Your item is a Laptop");
    }
    else if grade > 70.0 && grade <=100.50
    {
        println!("Your item code is a laptop")
    }   

}
