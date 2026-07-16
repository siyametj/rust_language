// constants.rs

pub fn constants() {
    // Syntax
    // const VARIABLE_NAME: dataType = value;

    // 1. Constant Naming Convention
    const USER_LIMIT: i8 = 100;
    const PI_VALUE: f32 = 3.1416;
    // Display value of the constants
    println!("User limit: {USER_LIMIT} and PI value: {PI_VALUE}");

    // Can't change!!!!!!
    const PLAYER_ID: u32 = 77348;
    // const PLAYER_ID: u32 = 9121;
    println!("Player id (constant): {PLAYER_ID}");

}
