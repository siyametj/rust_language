// borrowing_and_references.rs

pub fn borrowing_and_references() {
    // Immutable Reference (&)
    let pizza: String = String::from("Pepperoni Pizza");
    let ref_1: &String = &pizza; // read only
    let ref_2: &String = &pizza; // read only
    let ref_3: &String = &pizza; // read only

    println!("Reference 1: {ref_1}\nReference 2: {ref_2}\nReference 3: {ref_3}");
    println!("Main string: {pizza}\n");

    // Mutable Reference (&mut)
    let mut my_score: i32 = 100;
    add_bonus_poin(&mut my_score);
    println!("Score: {my_score}\n");

    let mut text: String = String::from("Aria Love");
    let ref1: &String = &text; // read only
    let ref2: &String = &text; // read only
    // let ref3: &mut String = &mut text; // it not work
    println!("Reference 1: {ref1}\nReference 2: {ref2}");

    let ref3: &mut String = &mut text;
    ref3.push_str(" pizza");
    println!("After modify text: {ref3}");

}

fn add_bonus_poin(point: &mut i32) {
    *point += 50
}
