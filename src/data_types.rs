// data_types.rs

pub fn data_types() {
    let student_name = "Siyam"; // string type data
    let rate = 9.88; // float type data
    let is_student = true; // bolean type data

    println!("Student name is {student_name}"); // It is modern style for print
    println!("Rate is {}", rate); // It is clasic style for print
    println!("Is student? {}\n", is_student); // \n used for extra line

    // Scalar type data
    // 1. Integer
    let result = 10; // i32 by default
    let sum: i32 = 12 - 30;
    let age: u32 = 17; // Only possitive allowed
    let mark: isize = 30; // Size not fix + negetive or positive
    let index: usize = 20; // Size not fix + only positive
    println!("result: {result}\nSum: {sum}\nAge: {age}\nMark: {mark}\nIndex: {index}");

    // Integer overflow
    let max_int: u8 = 255; // Its max range of u8
    println!("Max range is 0 to {max_int}");
    // 0 - 255 only allowed for u8
    // let x1: u8 = 256; // Overflow value is 0
    // let x2: u8 = 257; // Overflow value is 1
    // println!("First value is {} and Seconf value is {}", x1, x2);


    // 2. Floating-Point
    let result = 10.0; // f64 by default
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600;
    println!("Result: {} - Interest ${} - Cost ${:.3}", result, interest, cost);

    // 3. Booleans
    let is_siyam: bool = true;
    let is_aria = false;
    println!("Is siyam? {is_siyam}");
    println!("Is Aria? {is_aria}");

    // 4. Characters
    let grade: char = 'A'; // Use single quote
    println!("Grade is {grade}");
}
