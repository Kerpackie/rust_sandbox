#[derive(Debug)] // Allows us to debug the enum - Derive Macro
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    // We can have an enum return a generic!
    Ok(T), // we can return types inside the enum.
    Err(E),
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T),
}

fn create_car_colour_blue() -> CarColour {
    // Use -> to define return type.
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour // to return a value we can skip the return, and just have no semi-colon.
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    // We return the value, or a string with the rror
    if num_check < 5 {
        GivenResult::Ok(num_check) // return the value
    } else {
        GivenResult::Err("Not under 5!".to_string()) // return the error
    }

    // This is very like the ServiceResponse<T> pattern that I like to use, but, cleaner?
    // I like it.
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

// Using the built in rust result.
// We don't need the prefix!
fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check) // return the value
    } else {
        Err("Not under 5!".to_string()) // return the error
    }
}

fn remainder_zero_build_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums() {
        let car_colour: CarColour = create_car_colour_blue();
        dbg!(car_colour);

        let under_fix_res: GivenResult<u8, String> = check_under_five(2);
        dbg!(under_fix_res);

        let under_fix_res: GivenResult<u8, String> = check_under_five(7);
        dbg!(under_fix_res);

        let remainder: GivenOption<f32> = remainder_zero(12.2);
        dbg!(remainder);

        let remainder: GivenOption<f32> = remainder_zero(10.0);
        dbg!(remainder);

        let under_fix_res_built_in: Result<u8, String> = check_under_five_built_in(2);
        dbg!(under_fix_res_built_in);

        let under_fix_res_built_in: Result<u8, String> = check_under_five_built_in(7);
        dbg!(under_fix_res_built_in);

        let remainder_built_in: Option<f32> = remainder_zero_build_in(12.2);
        dbg!(remainder_built_in);

        let remainder_built_in: Option<f32> = remainder_zero_build_in(10.0);
        dbg!(remainder_built_in);
    }
}
