// tuple.rs

pub fn tuple() {
    // Syntax 1
    // let tuple_name: (data_type_1, data_type_2, data_type_3) = (value_1, value_2, value_3);

    // Syntax 2
    // let tuple_name = (value_1, value_2, value_3);

    // Example 1:
    let my_new_tuple: (i32, &str, bool) = (12, "siyam", true);
    println!("My tuple: {:?}", my_new_tuple);

    // Example 2:
    let my_new_tuple = (17, "Siyam", true);
    println!("New tuple: {:?}", my_new_tuple);

    // Indexing of tuple
    println!("First item: {:?}", my_new_tuple.0);
    println!("Second item: {:?}", my_new_tuple.1);
    println!("Third item: {:?}\n", my_new_tuple.2);

    // The following example passes a tuple as parameter to a function. Tuples are passed by value to functions.
    print_tuple(my_new_tuple);
    println!();

    // Destructing
    let (age, name, is_student) = my_new_tuple;
    println!("Age: {age}\nName: {name}\nIs Student? {is_student}");

    // Mutable tuple
    let mut my_new_tuple: (i32, String, bool) = (17, String::from("Siyam"), true);
    my_new_tuple.1.push_str(" Bro");
    println!("Updated name: {:?}\n", my_new_tuple.1);

    // The Empty Tuple / Unit Type
    let empty_tuple: () = ();
    println!("The empty tuple: {:?}", empty_tuple);

    // Skip unecessary item
    let my_new_tuple = ("Siyam", "bro", "unecessary");
    let (first_name, last_name, _) = my_new_tuple;
    println!("First name: {first_name} - Last name: {last_name}");

    // Nested tuple
    let my_nested_tuple = (
        (18, "Aria", false),
        (17, "Siyam", true)
    );
    let (first, second) = my_nested_tuple;
    println!("Name: {:?} - Age: {:?} - IS student? {:?}", first.1, first.0, first.2); // Debug from (e.g., "Aria")
    println!("Name: {} - Age: {} - IS student? {}\n", second.1, second.0, second.2); // Display from (e.g., Siyam)

    // Return tuple
    println!("Name: {} | Age: {}", student_info().0, student_info().1);

    // Single Item Tuple
    let single_item = ("Siyam Bro",); // It's a tuple
    // let not_tuple = ("Siyam Bro"); // It's a normal &str
    println!("Single item look: {:?}\n", single_item);

    // Destructuring with Rest Pattern
    let long_tuple = ("siyam", "coder", "aria", "bro");
    let (name1, .., name2) = long_tuple;
    println!("Name: {} {}", name1, name2);

}

fn print_tuple(x: (i32, &str, bool)) {
    println!("Inside of tuple item: \n{:?}", x);
}

fn student_info() -> (&'static str, i32) {
    ("Siyam", 17) // Return this
}
