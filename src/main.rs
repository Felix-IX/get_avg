use std::io;

fn main() {
        println!("type 3 numbers and the program will calculate the average");
        let i: [f64; 3] = get_input();
        let avg = cal_avg(i);
        println!("the average is: {}", avg);
}

//* This function prompts the user to enter three numbers and returns them as a fixed-size array.
fn get_input() -> [f64; 3] {
        let mut input_array: [f64; 3] = [0.0; 3];
        for i in 0..3 {
                println!("please enter the {}{} number", i + 1, ordinal_suffix(i + 1));

                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).expect("failed to read line");

                match input_line.trim().parse::<f64>() {
                        Ok(num) => input_array[i] = num,
                        Err(_) => panic!("invalid input"),
                }
        }
        input_array
}

//* A helper function that calculates the average of an array containing three floating-point numbers.
fn cal_avg(ls: [f64; 3]) -> f64 {
        let avg: f64 = (ls[0] + ls[1] + ls[2]) / 3.0;
        avg
}

//* A helper function provides the suffix of the number
fn ordinal_suffix(num: usize) -> &'static str {
        match num % 10 {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
        }
}
