// string.rs

pub fn string() {
    /*
    Two type of string -----

    String Literal(&str)

    String Object(String)

    */

    // 1. String Literal
    let my_name = "Siyam"; // its &str
    let her_name: &str = "Aria"; // its same &str
    println!("My name is {my_name} and her name is {her_name}\n");

    // 2. Lifetime
    let my_name: &'static str = "Siyam-Bro";
    println!("My name is {my_name}\n");

    // 3. String Object
    // Syntax
    // String::new()  <-- For create emprty String
    // String::from() <-- Store some value
    let empty_string: String = String::new(); // empty
    println!("Empty String look: {empty_string}");
    println!("Empty String size: {}", empty_string.len()); // 0

    let full_string: String = String::from("Siyam");
    println!("Full String look: {full_string}");
    println!("Full String size: {}\n", full_string.len()); // 5

    // ## Common Methods - String Object
    // a) new()
    // An empty string object is created using the new() method and its value is set to hello.
    let mut empty_string: String = String::new(); // empty
    println!("Before change: {empty_string}");
    empty_string.push_str("hello"); // add new item
    println!("After change: {empty_string}\n"); // hello

    // b) to_string()
    // To access all methods of String object, convert a string literal to object type using the to_string() function.
    let string_name: &str = "Siyam-Bro"; // &str
    println!("Before change in String: {string_name}");
    let string_name :String = string_name.to_string(); // String
    println!("After change in String: {string_name}\n");

    // c) replace()
    // The replace() function takes two parameters − the first parameter is a string pattern to search for and the second parameter is the new value to be replaced.
    let name1 = "Miss Aria".to_string();
    let name1 = name1.replace("Miss", "");
    println!("After replace name: {name1}");
}
