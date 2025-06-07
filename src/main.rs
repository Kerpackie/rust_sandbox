const WELCOME_CONST: &str = "Stay around and play!"; // Const stored in static memory and on stack.


fn main() {
    println!("Welcome to the sandbox, {}", WELCOME_CONST);

    // Stack
    let x: i32 = 2;
    println!("The value of x is: {}", x);

    // Alternate line formatting
    let y: i32;
    y = 4;
    println!("The value of y is {}", y);

    // For loop
    for i in 0..=y { // 0..y for < ..= is <= in C loop
        if i != 4 {
            print!("{}, ", i) // note print macro instead of println macro.
        } else {
            println!("{} ", i) // println! will terminate with a new line, but wont start with one.
        }
    }

    // mutate a value.
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10; // change z
    println!("but is now {}", z);

    let freezing_temp: f64 = -2.4; // float can't be 2, has to be 2.x
    println!("freezing_temp is {}", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("is_zero_remainder is {}", is_zero_remainder);

    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let ascii_char: char = '😎';
    println!("ascii_char is {}", ascii_char);

    let my_floats: [f32; 10] = [0.0; 10]; // define array of float32, qty 10, then assign 0.0 to each position.
    println!("my_floats is {:?}", my_floats); // :? is needed for printing,

    let my_ints: [i32; 3] = [1, 2, 3]; // manually assign each value
    println!("my_ints is {:#?}", my_ints); //  or :#? for pretty printing

    let my_floats_new: [f32; 10] = my_floats.map(|n|n + 2.0); // || are closures, they seem to be like lambdas eg, n => n + 2.0
    println!("my_floats_new is: {:?}", my_floats_new);
    
}
