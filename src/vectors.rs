// vectros.rs

pub fn vectros() {
    // Creating a vector
    let mut my_vector: Vec<&str> = vec!["Miss Aria", "Siyam", "Bro"];
    println!("My vector: {:?}", my_vector);

    // Access Vector Elements
    println!("Her name: {}", my_vector[0]);
    println!("My name: {} {}", my_vector[1], my_vector[2]);

    let index_of_99: &str = match my_vector.get(99) {
        Some(s) => s,
        _ => "Index 99 is not in range",
    };
    println!("Unknown item: {}",index_of_99);

    //  Change Array Values
    let mut my_array: Vec<i32> = vec![10, 20, 30, 40];
    my_array[0] = 100; // 10 -> 100
    println!("My array: {:?}", my_array);

    // Array Length
    println!("Length of array: {}", my_array.len());
    println!();

    // Loop Through an Array
    println!("Array: {:?}", my_vector);
    for name in &my_vector {
        println!("Name: {}", name);
    }
    println!();

    // Add new item
    my_vector.push("Miss. Aria");
    my_vector.push("Siyam Bro");
    println!("Vector after add new item: {:?}", my_vector);

    // Remove last item from vector
    my_vector.pop(); // remove last item
    println!("Vector after remove last item: {:?}", my_vector);

    // Chnage value of vector by loop
    let mut exam_result: Vec<i8> = vec![23, 45, 67, 34, 45, 78];
    println!("Exam result: {:?}", exam_result);
    // lets add +5
    for num in exam_result.iter_mut() {
        *num += 5;
    }
    println!("After add +5: {:?}", exam_result);
    println!();

    // Check any item inside of vector or not
    println!("'Siyam' in my_vector: {}", my_vector.contains(&"Siyam"));
    // &str + & mean - pointer of this &str

    // Is vector empty?
    println!("Is my_vector empty? {}", my_vector.is_empty()); // false
    println!();

    // Remove a specific item
    my_vector.remove(1); // remove Siyam
    println!("After remove: {:?}", my_vector);

    // Filter (Remove which not maintain condition)
    let mut stock: Vec<i32> = vec![199, 23, 34, 566, 3423, 3423];
    println!("Stock: {:?}", stock);

    stock.retain(|&x| x > 200); // remove item which under 200
    println!("After remove: {:?}", stock);

    // Vector slicing
    let stock: Vec<i32> = vec![199, 23, 34, 566, 3423, 3423];
    let highest_stock = &stock[3..6];
    println!("Stock: {:?}\nHighest stock: {:?}", stock, highest_stock);
}
