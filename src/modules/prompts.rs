use crate::modules::helper::{cal_avg, ordinal_suffix};
use colored::Colorize;

pub fn greetings() {
    println!(
        "{}{} and the program will calculate the average",
        "
    

 ██████   ███████  ████████            █████   ██    ██   ██████    ██ 
██        ██          ██              ██   ██  ██    ██  ██         ██ 
██   ███  █████       ██               ███████  ██    ██  ██   ███   ██ 
██    ██  ██          ██                ██   ██   ██  ██   ██    ██     
 ██████   ███████     ██     ███████    ██   ██    ████     ██████    ██ 
                                                               

"
        .blue(),
        "Input numbers".green()
    );
}

pub fn ask_for_number(i: usize) {
    println!("\nPlease enter the {}{} number, or {} to stop", i + 1, ordinal_suffix(i), "z".bright_red());
}

pub fn invalid_number() {
    println!("{} did you mean {}?\n", "Invalid number!".red(), "z".green());
}

pub fn no_number_detected() {
    println!("{}", "Please enter at least one number".yellow());
}

pub fn the_avg(v: &Vec<f64>) {
    println!("{}{}", "The average is: ".green(), cal_avg(v))
}
