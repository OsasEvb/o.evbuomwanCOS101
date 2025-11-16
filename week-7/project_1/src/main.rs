use std::io;

// Function to read a number from user
fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}

// Functions for each formula
fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_rhombus(d1: f64, d2: f64) -> f64 {
    0.5 * d1 * d2
}

fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_cube(side: f64) -> f64 {
    6.0 * side * side
}

fn volume_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("Choose the shape you want to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_number("Enter your choice (1–5): ") as u32;

    match choice {
        1 => {
            let h = read_number("Enter height:");
            let b1 = read_number("Enter base1:");
            let b2 = read_number("Enter base2:");
            let result = area_trapezium(h, b1, b2);
            println!("Area of Trapezium = {}", result);
        }

        2 => {
            let d1 = read_number("Enter diagonal 1:");
            let d2 = read_number("Enter diagonal 2:");
            let result = area_rhombus(d1, d2);
            println!("Area of Rhombus = {}", result);
        }

        3 => {
            let base = read_number("Enter base:");
            let altitude = read_number("Enter altitude:");
            let result = area_parallelogram(base, altitude);
            println!("Area of Parallelogram = {}", result);
        }

        4 => {
            let side = read_number("Enter length of the side:");
            let result = area_cube(side);
            println!("Area of Cube = {}", result);
        }

        5 => {
            let radius = read_number("Enter radius:");
            let height = read_number("Enter height:");
            let result = volume_cylinder(radius, height);
            println!("Volume of Cylinder = {}", result);
        }

        _ => println!("Invalid choice!"),
    }
}


