// loops.rs

pub fn loops() {
    /*
    Type of loop

    - while
    - loop
    - for
    */

    // 1) For Loop
    for i in 1..4 { // show 1 to 3 (not 4)
        println!("{i}. Hi bro.");
    }
    println!();

    for i in 1..=5 { // show 1 to 5
        println!("{i}. Hi Aria!");
    }
    println!();

    // Loop throw in a vector
    let my_vec: Vec<&str> = vec!["Rustaceans", "Aria", "Siyam"];
    for name in my_vec.iter() {
        println!("Name - {name}");
    }
    println!();

    // Count reverse
    for num in (1..=5).rev() {
        println!("Count... {num}");
    }
    println!("Booooom!");
    println!();

    // 2) loop
    let mut number: i32 = 1;
    loop {
        println!("[{number}] It's rust.");
        if number >= 5 {
            println!("Loop end!");
            break;
        }

        number += 1;
    };
    println!();

    // 3) While loop
    let mut new_num: i32 = 10;
    while new_num <= 12 {
        println!("Number is now: {new_num}");
        new_num += 1;
    }

}
