#[cfg(test)]
mod tests {

    macro_rules! mad_skills {
        /*        ($x: expr) => {
            format!("You sent an expression: {}", $x)
        };*/
        ($x: ty) => {
            match stringify!($x) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent something else".to_string(),
            }
        };
    }

    macro_rules! custom_vec {
        ($($x: expr), +) => {
            // Use a code block to prevent leaking variables out of the scope.
            {
                let mut temp_vec = Vec::new();

                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        };
    }

    #[test]
    fn tests_declaritive_macro_1() {
        /*        println!("Hello 1");
                dbg!("Hello 2");
                let x: Vec<i32> = vec!(1, 2, 3);
                let formatted: String = format!("Hello 3 with vec {:?}", x);
                dbg!(formatted);
        */

        let some_var: String = mad_skills!(String);
        dbg!(some_var);

        let mut x: Vec<i32> = vec!();
        dbg!(x);
        
        let mut y: Vec<i32> = custom_vec!(1);
        dbg!(y);
        
    }
}
