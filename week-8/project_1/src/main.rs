use std::io;

// Define staff categories
#[derive(Debug)]
struct RoleLevel {
    aps_level: &'static str,
    office_admin: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

fn main() {
    // Table using a vector of structs
    let table = vec![
        RoleLevel { aps_level: "APS 1-2", office_admin: "Intern", academic: "-", lawyer: "Paralegal", teacher: "Placement" },
        RoleLevel { aps_level: "APS 3-5", office_admin: "Administrator", academic: "Research Assistant", lawyer: "Junior Associate", teacher: "Classroom Teacher" },
        RoleLevel { aps_level: "APS 5-8", office_admin: "Senior Administrator", academic: "PhD Candidate", lawyer: "Associate", teacher: "Snr Teacher" },
        RoleLevel { aps_level: "EL1 8-10", office_admin: "Office Manager", academic: "Post-Doc Researcher", lawyer: "Senior Associate 1-2", teacher: "Leading Teacher" },
        RoleLevel { aps_level: "EL2 10-13", office_admin: "Director", academic: "Senior Lecturer", lawyer: "Senior Associate 3-4", teacher: "Deputy Principal" },
        RoleLevel { aps_level: "SES", office_admin: "CEO", academic: "Dean", lawyer: "Partner", teacher: "Principal" },
    ];

    println!("\n--- APS LEVEL CHECKER ---\n");
    
    // Get profession
    println!("Enter profession (office_admin, academic, lawyer, teacher): ");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).unwrap();
    let profession = profession.trim().to_lowercase();

    // Get role title
    println!("Enter staff job title: ");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    // Get years of experience
    println!("Enter years of experience: ");
    let mut exp = String::new();
    io::stdin().read_line(&mut exp).unwrap();
    let years: u32 = exp.trim().parse().unwrap_or(0);

    // Determine APS level
    let mut result = None;

    for row in &table {
        let match_title = match profession.as_str() {
            "office_admin" => row.office_admin,
            "academic" => row.academic,
            "lawyer" => row.lawyer,
            "teacher" => row.teacher,
            _ => {
                println!("Unknown profession.");
                return;
            }
        };

        // Skip if table has "-" (not applicable)
        if match_title == "-" {
            continue;
        }

        if match_title.eq_ignore_ascii_case(title) {
            result = Some(row.aps_level);
            break;
        }
    }

    println!("\nResult:");
    match result {
        Some(level) => {
            println!("The staff member holds APS level: {}", level);

            // Optional: match example from prompt
            if level == "APS 5-8" && years >= 5 && years <= 8 {
                println!("✔ Valid: matches APS 5–8 for 5–8 years experience.");
            }
        }
        None => println!("No APS level found for the given job title and profession."),
    }
}
