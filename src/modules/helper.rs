use crate::modules::prompts::{no_number_detected, the_avg};

//* A helper function that calculates the average of a vector containing multiple floating-point numbers.
pub fn cal_avg(v: &Vec<f64>) -> f64 {
    let avg: f64 = v.iter().sum::<f64>() / v.len() as f64;
    avg
}

pub fn check_and_print(v: &Vec<f64>) {
    if v.is_empty() {
        // Please enter at least one number
        no_number_detected()
    } else {
        // The average is: XXX
        the_avg(v)
    }
}

//* A utility function provides the suffix of the number
pub fn ordinal_suffix(num: usize) -> &'static str {
    match (num + 1) % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}
