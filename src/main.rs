mod enums;

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

// Closures

fn closures()
{
    let num: i32 = 5;
    let add_num = |x: i32| x + num; // closures are similar to lambda or Func<int, int> in C\
    let new_num: i32 = add_num(7);
    
    dbg!(new_num);
}

// Number literals - from the Rust Book.
fn number_literals()
{
    println!("Big Number is: {}", 98_222_000); // use underscores to deliminate and split big numbers
    println!("hex is {}", 0xff);
    println!("Octal is {}", 0o77);
    println!("Binary is {}", 0b1111_0000);
    println!("Bytes 'A' is {}", b'A');
    
    // Raw - String Literal
    let text: &str = r#"{message : "Rust is awesome!"}"#; // Raw string litterals are like using @ in C#
    dbg!(text);
}

// Binary
fn binary(){
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    
    println!("a's value is {}", a);
    println!("b's value is {}", b);
    
    println!("a in binary {:08b}", a);
    println!("b in binary {:08b}", b);
    
    // Logic Gates
    println!("AND {:08b}", a & b);
    println!("OR {:08b}", a | b);
    println!("XOR {:08b}", a ^ b);
    println!("NOT {:08b}", !a);
    
    // Bitwise Operations
    
    // Bit Shift
    println!("a's value is {}", a);
    println!("a in binary {:08b}", a);
    println!("a << 1 {:08b}", a << 1); // Bit shift left 1
    println!("a's value is {}", a << 1); // 84
    println!("a >> 1 {:08b}", a >> 1); // bit shift right 1
    println!("a's value is {}", a >> 1); // 85
    
    // Little and Big Endian
    let n: u16 = 0x1234;
    println!("n is: {:?}", n);
    
    let big_endian = n.to_be_bytes();
    let little_endian = n.to_le_bytes();
    
    println!("n in big endian: {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!("n in little endian: {:02X}{:02X}", little_endian[0], little_endian[1])
}

fn main() {
    println!("Welcome to the sandbox, {}", WELCOME_CONST);
    
    simple_variables();
    
    simple_arrays();
    
    strings();
    
    vectors_and_collections();
    
    closures();
    
    number_literals();
    
    binary();
}
