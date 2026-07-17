// operators.rs

pub fn operators() {
    /*
    Type of operators ------
    - Arithmetic
    - Bitwise
    - Comparison
    - Logical
    - Bitwise
    - Conditional
    */

    // ## Arithmetic Operators

    let number1: i8 = 20;
    let number2: i8 = 30;

    println!("Number1: {number1}\nNumber2: {number2}");

    // 1) Addition
    println!("Addition (+): {}", number1 + number2);

    // 2) Subtraction
    println!("Subtraction (-): {}", number1 - number2);

    // 3) Multiplication
    println!("Multiplication (*): {}", number1 as i32 * number2 as i32);

    // 4) Division
    println!("Division (/): {}", number1 / number2);

    // 5) Modulus
    println!("Modulus (%): {}\n", number1 % number2);

    // The ++ and -- operators are not supported in Rust.

    // ## Relational Operators

    let number1: i8 = 100;
    let number2: i8 = 126;
    println!("Number1: {number1}\nNumber2: {number2}");

    // 1) Greater than
    println!("100 is greater than 126 (100 > 126): {}", number1 > number2); // false
    println!("1 is greater than 1 (1 > 1): {}", 1 > 1); // false

    // 2) Less than
    println!("100 is less than 126 (100 < 126): {}", number1 < number2); // true
    println!("1 is less than 1 (1 < 1): {}", 1 < 1); // false


    // 3) Greater than or equal to
    println!("100 is greater than or equal to 126 (100 >= 126): {}", number1 >= number2); // false
    println!("1 is greater than or equal to 1 (1 >= 1): {}", 1 >= 1); // true

    // 4) Less than or equal to
    println!("100 is less than or equal to 126 (100 <= 126): {}", number1 <= number2); // true
    println!("1 is less than or equal to 1 (1 <= 1): {}", 1 <= 1); // true

    // 5) Equality
    println!("100 is equal to 126 (1 == 126): {}", number1 == number2); // false
    println!("1 is equal to 1 (1 == 1): {}", 1 == 1); // true

    // 6) Not equal
    println!("100 is not equal to 126 (100 != 126): {}", number1 != number2); // true
    println!("1 is not equal to 1 (1 != 1): {}\n", 1 != 1); // false

    // ## Logical Operators

    let my_name: &str = "Siyam";
    let my_age: i8 = 17;

    println!("My name is {} and i am {} years old", my_name, my_age);

    // 1) AND operator (&&)
    println!("Name: Siyam and age is 18? {}", my_name == "Siyam" && my_age == 18); // false

    // 2) OR operator (||)
    println!("Name: Siyam and age is 17? {}", my_name == "Siyam" || my_age == 18); // true

    // 3) NOT operator (!)
    println!("Name: not siyam? {}", my_name != "Siyam"); // false

}
