use crate::modules::helper::ordinal_suffix;
use std::io;
use colored::Colorize;

//* this function reads a number from the user and returns a tuple (bool, bool)
//* the first value is whether the user entered a valid number,
//* the second value is whether the user entered "z" to quit.
pub fn read_number(input_line: &str, input_vec: &mut Vec<f64>) -> (bool, bool) {
    let trimmed: &str = input_line.trim();
    if trimmed == "z" {
        return (false, true);
    }

    match trimmed.parse::<f64>() {
        Ok(num) => {
            input_vec.push(num);
            (true, false)
        }
        Err(_) => (false, false),
    }
}

//* This function prompts the user to enter multiple numbers and returns them as a vector.
pub fn get_input() -> Vec<f64> {
    let mut i: usize = 0;
    let mut input_vec: Vec<f64> = vec![];
    loop {
        // Please enter the XXX number, or z to stop
        println!("\nPlease enter the {}{} number, or {} to stop", i + 1, ordinal_suffix(i), "z".bright_red());

        let mut input_line: String = String::new();
        io::stdin().read_line(&mut input_line).expect("Failed to read line");

        let (is_valid, is_quit) = read_number(&input_line, &mut input_vec);
        if is_quit {
            return input_vec;
        } else if !is_valid {
            // Invalid number, did you mean z?\n
            println!("{} did you mean {}?\n", "Invalid number!".red(), "z".green());
        } else {
            i += 1;
        }
    }
}
