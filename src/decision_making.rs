// decision_making.rs

pub fn decision_making() {
    /*
    1) if statement
    An if statement consists of a Boolean expression followed by one or more statements.

    2) if...else statement
    An if statement can be followed by an optional else statement, which executes when the Boolean expression is false.

    3) else...if and nested ifstatement
    You can use one if or else if statement inside another if or else if statement(s).

    4) match statement
    A match statement allows a variable to be tested against a list of values.
    */

    // 1) if statement
    if true {
        println!("Rewrite it in rust. (RIIR)")
    }

    let name: &str = "bro";

    if name == "bro" {
        println!("It's bro");
    }

    // 2) else statement
    if name == "Siyam" {
        println!("It's siyam.");
    } else {
        println!("It's unknown person.");
    }

    // 3) Nested If
    let number: i32 = 200;

    if number > 0 {
        println!("{number} is positive.");
    } else if number < 0 {
        println!("{number} is negative.");
    } else {
        println!("Input is not a number.");
    }

    // # Match Statement
    let name: &str = "Siyam";

    let user_name: &str = match name {
        "Siyam" => "Siyam",
        "Bro" => "Bro",
        "Siyam Bro" => "Siyam Bro",
        _ => "Unknown Person",
    };
    println!("User: {user_name}");

}
