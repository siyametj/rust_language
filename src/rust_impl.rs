// rust_impl.rs

// Create a struct
struct Student {
    pub name: String, // not private
    gpa: f32, // private
    is_active: bool, // private
}

impl Student {
    // Take name only in initate
    fn new(name: String) -> Self {
        // gpa and is_active set as default
        Student { name, gpa: 0.0, is_active: false }
    }

    fn update_gpa(&mut self, new_gpa: f32) {
        self.gpa = new_gpa
    }

    fn show_student(&self) {
        println!("Student Info: \nName: {}\nGPA: {}\nActivity: {}", self.name, self.gpa, self.is_active)
    }
}

pub fn rust_impl() {
    let mut siyam = Student::new(String::from("Siyam"));
    siyam.name = "Siyam Bro".to_string(); // change public item
    siyam.update_gpa(4.11); // Change default value
    siyam.show_student();

}
