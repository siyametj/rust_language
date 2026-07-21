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
    println!("Name 1: {name1}\nName 2: {name2}");
}
