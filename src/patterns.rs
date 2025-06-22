#[cfg(test)]
mod test {
    use std::panic::panic_any;
    use ethers::types::Res;
    use super::*;
    
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