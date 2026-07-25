// rust_structs.rs

struct Student {
    name: String,
    age: u32,
    is_passed: bool
}

struct ToDOList {
    title: String,
    descripton: String,
}

impl ToDOList {
    fn new(title: String, descripton: String) -> ToDOList {
        ToDOList { title, descripton }
    }

    fn display(&self) -> () {
        println!("-> Title: {}\n-> Description: {}", self.title, self.descripton);
    }
}

pub fn rust_structs() {

    // Immutable
    let bro: Student = Student {
        name: String::from("Siyam"),
        age: 17,
        is_passed: false,
    };

    println!("Student name: {}", bro.name);
    println!("Student age: {}", bro.age);
    println!("Student passed?: {}\n", bro.is_passed);

    // Mutable
    let mut aria: Student = Student {
        name: String::from("Miss. Aria"),
        age: 21,
        is_passed: true,
    };
    aria.name = String::from("Aria"); // change here
    println!("Student name: {}", aria.name);
    println!("Student age: {}", aria.age);
    println!("Student passed?: {}\n", aria.is_passed);

    // Field Init Shorthand
    let new_student_1 = create_student("siyametz".to_string(), 17, true);
    println!("Name: {}\nAge: {}\nIs passed: {}\n", new_student_1.name, new_student_1.age, new_student_1.is_passed);

    let todo_1 = ToDOList::new("learn rust struct".to_string(), "Do a project".to_string());
    todo_1.display(); // methood

}

fn create_student(name: String, age: u32, is_passed: bool) -> Student {
    Student {
        name,
        age,
        is_passed,
    }
}
