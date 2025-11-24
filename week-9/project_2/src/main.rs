use std::fs::File;
use std::io::{self, Write};

struct Student {
    name: &'static str,
    matric: &'static str,
    department: &'static str,
    level: u32,
}

fn main() -> io::Result<()> {
    // Vector of students
    let students = vec![
        Student {
            name: "Oluchi Mordi",
            matric: "ACC10211111",
            department: "Accounting",
            level: 300,
        },
        Student {
            name: "Adams Aliyu",
            matric: "ECO10110101",
            department: "Economics",
            level: 100,
        },
        Student {
            name: "Shania Bolade",
            matric: "CSC10328828",
            department: "Computer",
            level: 200,
        },
        Student {
            name: "Adekunle Gold",
            matric: "EEE10202002",
            department: "Electrical",
            level: 200,
        },
        Student {
            name: "Blanca Edomeh",
            matric: "MEE10202001",
            department: "Mechanical",
            level: 100,
        },
    ];

    // Display the details
    println!("PAU SMIS");
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric Number", "Department", "Level");

    for s in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric, s.department, s.level
        );
    }

    // Save to file
    let mut file = File::create("students.txt")?;

    writeln!(file, "PAU SMIS\n")?;
    writeln!(
        file,
        "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level"
    )?;

    for s in students {
        writeln!(
            file,
            "{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric, s.department, s.level
        )?;
    }

    println!("\nFile 'students.txt' created successfully!");
    Ok(())
}
