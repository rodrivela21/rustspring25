// Create a struct Student (major)
struct Student {
    major: String,
}

// First-order function: assigns a major to student
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

// Higher-order function: updates all students' majors using a list of new majors
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String), new_majors: Vec<String>) -> Vec<Student> {
    for (student, new_major) in collection.iter_mut().zip(new_majors.into_iter()) {
        behavior(student, new_major);
    }
    collection
}

// Helper function to print student majors
fn print_majors(students: &Vec<Student>) {
    println!("Student Majors:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}: {}", i + 1, student.major);
    }
}

fn main() {
    // Create a vector of students with predefined different majors
    let students = vec![
        Student { major: String::from("Engineering") },
        Student { major: String::from("Biology") },
        Student { major: String::from("Philosophy") },
    ];

    println!("Before updating majors:");
    print_majors(&students);

    // New majors to assign
    let new_majors = vec![
        String::from("Mathematics"),
        String::from("Chemistry"),
        String::from("Literature"),
    ];

    // Update students with new majors
    let updated_students = update_majors(students, assign_major, new_majors);

    println!("\nAfter updating majors:");
    print_majors(&updated_students);
}
