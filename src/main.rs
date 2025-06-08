const WELCOME_CONST: &str = "Stay around and play!"; // Const stored in static memory and on stack.

fn simple_variables()
{
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
}

fn simple_arrays()
{
    let my_floats: [f32; 10] = [0.0; 10]; // define array of float32, qty 10, then assign 0.0 to each position.
    println!("my_floats is {:?}", my_floats); // :? is needed for printing,

    let my_ints: [i32; 3] = [1, 2, 3]; // manually assign each value
    println!("my_ints is {:#?}", my_ints); //  or :#? for pretty printing

    let my_floats_new: [f32; 10] = my_floats.map(|n|n + 2.0); // || are closures, they seem to be like lambdas eg, n => n + 2.0
    println!("my_floats_new is: {:?}", my_floats_new);
}

fn strings()
{
    let name: &str = "Cillian"; //String literal - use & to act as a reference to heap location.

    println!("name is {:?}", name);

    let dynamic_name: String = String::from("Cillian Keogh");
    println!("dynamic name is {:?}", dynamic_name);
    println!("My dynamic name stored in memory {:p}", &dynamic_name); // :p is a pointer

    let dynamic_name: String = name.to_string(); // this is the same as doing String = String::from();
    println!("dynamic name is {:?}", dynamic_name);
    let dynamic_name: String = "Cillian Keogh".to_string();
    println!("dynamic name is {:?}", dynamic_name);

    // Get a slice of 7 characters.
    let str_slice: &str = &dynamic_name[0..7];
    println!("str_slice is {:?}", str_slice);
}

fn vectors_and_collections()
{
    // Vectors!
    let mut chars: Vec<char> = Vec::new();
    // Insert to a position in the vector
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');

    // Push to next position in vector
    chars.push('l');
    chars.push('o');
    chars.push('.');

    println!("chars is {:?}", chars);

    dbg!(&chars); // debug macro - we need to use the reference to prevent change of ownership to the debug statement

    let removed_char: char = chars.pop().unwrap(); // expects an option enum by default, so we iwll use unwrap.
    println!("removed char is {:?}", removed_char);
    println!("chars is {:?}", chars);

    chars.iter().for_each(|c| println!("char is {}", c)); // define an itterator, and the foreach.

    let chars_again: Vec<char> = vec!('h', 'e', 'l', 'l', 'o'); // Alternate way to create a vector.
    dbg!(&chars_again); // use the reference in-case we want to use it again.

    let collected: String = chars_again.iter().collect(); // Iterate over and collect them all.
    dbg!(collected);

    // What would happen if we did it with ints?
    //let int_vec: Vec<i32> = vec!(1, 2, 3, 4);
    //let int_vec_collected: i32 = int_vec.iter().collect();
    // It does not allow us to do this! Interesting!
    
    for c in chars_again{
        print!("{}", c);
        if c == 'o' { 
            println!(", world!");
        }
    }
}



fn main() {
    println!("Welcome to the sandbox, {}", WELCOME_CONST);

    vectors_and_collections();
}
