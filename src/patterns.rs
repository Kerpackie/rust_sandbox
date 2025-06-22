
#[derive(Debug)]
enum Message {
    Quit,
    ChangeColour(i32, i32, i32),
    Move { x: i32, y: i32},
    Write(String)
}

fn process_message(msg: Message){
    match msg {
        Message::Quit => {
            println!("I Quit!")
        },
        Message::ChangeColour(red, green, blue) => {
            println!("Red: {}, Green: {}, Blue: {}", red, green, blue)
        },
        Message::Move { x, y: new_name } => {
            println!("X is {}, Y is as new_name is {}", x, new_name)
        }
        Message::Write(text) => {
            println!("{}", text)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_match_guard() {
        let pair = (2, -2);
        match pair { 
            (x, y) if x == y => println!("They match!"),
            (x, y) if x + y == 0 => println!("They neutralize"),
            (_, y) if y == 2 => println!("Y is indeed +2"),
            _ => println!("We are not bothered")
        };
    }

    #[test]
    fn tests_match_struct() {
        struct Location {
            x: i32,
            y: i32
        }
        
        let location = Location { x: 0, y: 20 };
        
        match location { 
            Location {x, y: 0 } => println!("Y is on the axis"),
            Location {x: 0, y} => println!("X is on the axis"),
            Location {x, y} => println!("X and Y are not on the axis"),
        };
    }

    #[test]
    fn tests_large_enum() {
        let my_quit: Message = Message::Quit;
        process_message(my_quit);

        let my_colour: Message = Message::ChangeColour(10, 20, 255);
        process_message(my_colour);

        let my_move: Message = Message::Move {x: 10, y: 100};
        process_message(my_move);

        let my_write: Message = Message::Write("My awesome string".to_string());
        process_message(my_write);
    }

    #[test]
    fn tests_match_literals() {
        let number :i32 =  20;

        // Match is like switch on steroids.
        // Can extract values out of enums, can id what a variable is and perfrom a task based on that.

        match number {
            1 => println!("It was the first!"),
            2 | 3 | 5 | 7 | 15 | 20 => println!("We found it in the sequence!"),
            _ => println!("It was something else...")
        };

        // By default we can return a value contained within too.
        // This returns the string slice that is contained within.

        let res: &str = match number {
            1 => "It was the first!",
            2 | 3 | 5 | 7 | 15 | 20 => "We found it in the sequence!",
            _ => "It was something else..."
        };

        println!("{}", res);
    }

    #[test]
    fn tests_match_option() {
        let some_num: Option<i32> = Some(10);
        let prob_none: Option<i32> = None;

/*        let res: i32 = match some_num {
            Some(i) => i,
            None => {
                panic!("There was a problem");
            }
        };

        println!("{}", res);*/

        let my_int: i32 = if let Some(i) = some_num {
            i
        } else {
            panic!("There was a problem");
        };

        println!("My int: {}", my_int)
    }

    #[test]
    fn tests_match_result() {
        let some_result: Result<i32, String> = Ok(50);
        let some_error: Result<i32, &str> = Err("There was a problem");

        let res = match some_result {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        };

        println!("{}", res);


        // We can extract out the Ok/Err values too
        let my_int: i32 = if let Ok(i) = some_result {
            i
        } else {
            panic!("there was a problem")
        };

        println!("{}", my_int)
    }
}