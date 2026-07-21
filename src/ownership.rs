// ownership.rs

pub fn ownership() {
    let x: String = String::from("My pizza");
    let y = x;
    // println!("X: {x}");
    println!("Y: {y}");

    let number1: i32 = 100;
    let number2 = number1;
    println!("Number 1: {number1}");
    println!("Number 2: {number2}\n");

    // Clone
    let name1: String = String::from("Miss Aria");
    let name2: String = name1.clone();
    println!("Name 1: {name1}\nName 2: {name2}\n");

    // Function and ownership
    fn take_string(string: String) {
        println!("The input is about: {string}");
    } // string finish here

    let my_string: String = String::from("I like pizza!");
    // take_string function take ownership of my_string
    take_string(my_string);
    // println!("My string: {my_string}"); // its not alive
    println!();

    // Function and return
    fn give_ownership() -> String {
        let new_string: String = String::from("Aria love pizza!");
        new_string
    }
    let new_string: String = give_ownership();
    println!("New string: {new_string}\n");

    // Function with clone
    // give clone! Not main
    let my_string: String = String::from("Aria don't like burger!");
    my_func(my_string.clone());
    println!("My string: {my_string}"); // it works
}

fn my_func(string: String) {
    println!("New string: {string}");
} // string is finish here
