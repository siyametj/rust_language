// type_casting.rs

pub fn type_casting() {
    let my_int: i32 = 100; // An integer
    let my_float: f64 = my_int as f64; // i32 -> f64
    println!("Integer -> {my_int} and Float -> {my_float}");

    let deciaml: f64 = 99.99;
    let integer: i32 = deciaml as i32;
    println!("Deciaml Value -> {deciaml} and Integer -> {integer}\n");

    // Overflow!!!!!
    let my_big_number: i128 = 83868397492749;
    let my_small_number: i8 = my_big_number as i8; // Thats show only 13 (By warp for i8)
    println!("Big number is: {my_big_number} and Small number is: {my_small_number}");

    // Character to integer
    let alphabet: char = 'A';
    let ascii_value: i8 = alphabet as i8;
    println!("Alphabet is: {alphabet} and ASCII Value is: {ascii_value}"); // 65

    // String to number
    let string_number: &str = "100";
    let only_number: i8 = string_number.parse().expect("Can't convert this!");
    println!("String: {string_number} and Number: {only_number}"); // SUccessfully convert

    // let wrong_number: &str = "12s"; // Invalid digits
    // let new_number: i8 = wrong_number.parse().expect("Can't convert this!");
    // println!("Wrong number: {wrong_number} and Number: {new_number}");

    // Number separator
    // For read big number, use '_' ; Note: Its dont effect in print value
    let salary: i32 = 5_00_000; // integer
    let my_float: f64 = 23_34_566.783_00; // float
    println!("Salary: ${} and Float: {}", salary, my_float);
}
