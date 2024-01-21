use std::io;

fn main() {
        println!("Input numbers and the program will calculate the average");

        let v: Vec<f64> = get_input();
        let avg = match v.len() {
                0 => {
                        println!("Please enter at least one number");
                        return;
                }
                _ => cal_avg(v),
        };
        println!("The average is: {}", avg);
}

//* this function reads a number from the user and returns a tuple of (bool, bool)
//* the first value is whether the user entered a valid number,
//* the second value is whether the user entered "z" to quit.
fn read_number(input_line: &str, input_vec: &mut Vec<f64>) -> (bool, bool) {
        let trimmed = input_line.trim();
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
fn get_input() -> Vec<f64> {
        let mut i = 0;
        let mut input_vec: Vec<f64> = Vec::new();
        loop {
                println!("Please enter the {}{} number, or z to stop", i + 1, ordinal_suffix(i));

                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).expect("Failed to read line");

                let (is_valid, is_quit) = read_number(&input_line, &mut input_vec);
                if is_quit {
                        return input_vec;
                } else if !is_valid {
                        println!("Invalid number, did you mean z?\n");
                } else {
                        i += 1;
                }
        }
}

//* A helper function that calculates the average of a vector containing multiple floating-point numbers.
fn cal_avg(v: Vec<f64>) -> f64 {
        let avg: f64 = v.iter().sum::<f64>() / v.len() as f64;
        avg
}

//* A helper function provides the suffix of the number
fn ordinal_suffix(num: usize) -> &'static str {
        match (num + 1) % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
        }
}
