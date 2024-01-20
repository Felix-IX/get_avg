use std::io;

fn main() {
        println!("input numbers and the program will calculate the average");
        println!("enter 'z' to stop input");
        let v: Vec<f64> = get_input();
        let avg = cal_avg(v);
        println!("the average is: {}", avg);
}

//* This function prompts the user to enter three numbers and returns them as a fixed-size array.
fn get_input() -> Vec<f64> {
        let mut i = 0;
        let mut input_array: Vec<f64> = Vec::new();
        loop {
                println!("please enter the {}{} number, or z to stop", i + 1, ordinal_suffix(i));

                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).expect("failed to read line");

                if &input_line.trim().to_lowercase()[..] == "z" {
                        return input_array;
                }
                match input_line.trim().parse::<f64>() {
                        Ok(num) => input_array.push(num),
                        Err(_) => panic!("invalid input"),
                }
                i += 1;
        }
}

//* A helper function that calculates the average of an array containing three floating-point numbers.
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
