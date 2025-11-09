use std::io;

fn main() {
    println!("==============================");
    println!("        FOOD MENU");
    println!("==============================");
    println!("P = Poundo Yam/Edinkaiko Soup - ₦3,200");
    println!("F = Fried Rice & Chicken      - ₦3,000");
    println!("A = Amala & Ewedu Soup        - ₦2,500");
    println!("E = Eba & Egusi Soup          - ₦2,000");
    println!("W = White Rice & Stew         - ₦2,500");
    println!("==============================");

    println!("Enter the code for your order:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let choice = input.trim().to_uppercase();

    let (item, price) = match choice.as_str() {
        "P" => ("Poundo Yam/Edinkaiko Soup", 3200),
        "F" => ("Fried Rice & Chicken", 3000),
        "A" => ("Amala & Ewedu Soup", 2500),
        "E" => ("Eba & Egusi Soup", 2000),
        "W" => ("White Rice & Stew", 2500),
        _ => {
            println!("Invalid selection. Please choose a valid menu item.");
            return;
        }
    };

    println!("\nYou selected: {}", item);
    println!("Price: ₦{}", price);
    println!("==============================");
    println!("Thank you for your order!");
}
