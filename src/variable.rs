// variable.rs

pub fn variable() {
    /*
    RULE FOR VARIABLE ---------------------------------------------------------------------

    The name of a variable can be composed of letters, digits, and the underscore character.

    It must begin with either a letter or an underscore.

    Upper and lowercase letters are distinct because Rust is case-sensitive.

    */

    // ## Syntax
    // let variable_name = value; no type specified
    // let variable_name: dataType = value; type specified

    let fees = 999; // no type specified
    let salary: i32 = 80_000_000; // type specified
    println!("Fee: ${fees} and Salary: ${salary}");

    // 1. Immutable
    // By default, variables are immutable − read only in Rus
    let name: &str = "Siyam";
    println!("Name is {}\n", name);
    // name = "Aria"; it occured error

    // 2. Mutable
    // Variables are immutable by default. Prefix the variable name with mut keyword to make it mutable

    // let mut variable_name = value;
    // let mut variable_name: dataType = value;

    let mut student_name = "Siyam";
    println!("Student name is {student_name}");
    student_name = "Aria"; // Change value here
    println!("Changed to {student_name}");

    // 3. Shadowing
    let spaces = "   ";
    let spaces = spaces.trim();
    println!("{}", spaces);

}
