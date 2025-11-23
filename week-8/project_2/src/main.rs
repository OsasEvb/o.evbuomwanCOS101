use std::cmp::Ordering;

// Define a compound data type (struct)
#[derive(Debug)]
struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    // Sample list of developers (You may replace with interview input)
    let developers = vec![
        Developer {
            name: String::from("Alice"),
            years_of_experience: 5,
        },
        Developer {
            name: String::from("Bola"),
            years_of_experience: 7,
        },
        Developer {
            name: String::from("Chidi"),
            years_of_experience: 4,
        },
        Developer {
            name: String::from("Daniel"),
            years_of_experience: 10,
        },
    ];

    // Find developer with highest years of experience
    let most_experienced = developers
        .iter()
        .max_by(|a, b| a.years_of_experience.cmp(&b.years_of_experience))
        .unwrap();

    println!(
        "The developer with the highest years of experience is {} with {} years.",
        most_experienced.name, most_experienced.years_of_experience
    );
}
