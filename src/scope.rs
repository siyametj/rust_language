// scope.rs

pub fn scope() {
    // Inner scope
    let outer_var: String = String::from("I am global variable");
    println!("Global variable: {}", outer_var);

    {
        // Block scope
        let inner_var: String = String::from("I am inner");
        println!("Outer variable: {}", outer_var);
        println!("Inner variable: {}\n", inner_var);
    } // Inner are droped from RAM

    // Save borrow from scope
    let mut speed: i32 = 100;
    {
        let r1: &mut i32 = &mut speed;
        *r1 += 50;
        println!("Speed in scope: {}", r1);
    }
    println!("Speed in outside: {}", speed);
}
