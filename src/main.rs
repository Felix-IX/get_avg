use std::io;

fn main() {
        println!("Input numbers and the program will calculate the average");
        
        let v: Vec<f64> = get_input();
        if check_not_empty(&v) {
                let avg = cal_avg(v);
                println!("The average is: {}", avg);
        }
        else {
                println!("Please enter at least one number")
        }
}

//* This function prompts the user to enter multiple numbers and returns them as a fixed-size array.
fn get_input() -> Vec<f64> {
        let mut i = 0;
        let mut input_vec: Vec<f64> = Vec::new();
        loop {
                println!("Please enter the {}{} number, or q to stop", i + 1, ordinal_suffix(i));

                let mut input_line = String::new();
                match io::stdin().read_line(&mut input_line) {
                        Ok(_) => {
                                let trimmed = input_line.trim();
                                if trimmed == "q" {
                                        return input_vec;
                                }
                                match trimmed.parse::<f64>(){
                                        Ok(num) => {input_vec.push(num);
                                        i += 1;
                                },
                                        Err(_) => println!("Invalid number, did you mean q?")
                                }
                        }
                        Err(_) => panic!("Failed to read line"),
                }
        }
}

//* A helper function that calculates the average of an vector containing multiple floating-point numbers.
fn cal_avg(v: Vec<f64>) -> f64 {
        let avg: f64 = v.iter().sum::<f64>() / v.len() as f64;
        avg
}

//* A helper function checks if there's at least a number in the vector
fn check_not_empty(v: &Vec<f64>) -> bool {
        !v.is_empty()
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