// function.rs

pub fn function() {
    /*
    TOPIC OF FUNCTION: -----------------------------------------------------------------
    1) Defining a function
    A function definition specifies what and how a specific task would be done.

    2) Calling or invoking a Function
    A function must be called so as to execute it.

    3) Returning Functions
    Functions may also return value along with control, back to the caller.

    4) Parameterized Function
    Parameters are a mechanism to pass values to functions.
    */

    // 1) Defining a Function
    // syntax:
    // fn function_name(param1,param2..paramN) {
    //     function body
    // }
    fn say_hello() { // <----- making
        println!("Hello Aria!");
    }

    // 2) Calling or invoking a Function
    // syntax: function_name(val1,val2,valN)
    say_hello(); // <------ calling
    greet_aria();

    // 4) Returning Value from a Function
    /*
        With return statement
        Syntax 1:
        fn function_name() -> return_type {
        //statements
        return value;
        }

        Shorthand syntax without return statement
        Syntax 2:
        fn function_name() -> return_type {
        value //no semicolon means this value is returned
    }
    */
    println!("1 + 100 = {}", addition(1, 100));
    println!();

    // # Passing string to a function
    let my_string: String = String::from("I'ts Aria");
    display_string(my_string);

}

fn greet_aria() {
    println!("Hi Aria!"); // thats not Expression
}

fn addition(number1: i32, number2: i32) -> f32 {
    number1 as f32  + number2 as f32 // thats call Expression
}

fn display_string(string: String) {
    println!("----> {}", string);
}
