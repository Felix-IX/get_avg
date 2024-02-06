pub mod modules;
use modules::{helper::check_and_print, input::get_input};
use colored::Colorize;

fn main() {
    // GET_AVG
    // Input numbers and the program will calculate the average
    println!("{}{} and the program will calculate the average", "
    

 ██████   ███████  ████████            █████   ██    ██   ██████    ██ 
██        ██          ██              ██   ██  ██    ██  ██         ██ 
██   ███  █████       ██               ███████  ██    ██  ██   ███   ██ 
██    ██  ██          ██                ██   ██   ██  ██   ██    ██     
 ██████   ███████     ██     ███████    ██   ██    ████     ██████    ██ 
                                                               

".blue(), "Input numbers".green());

    let v: Vec<f64> = get_input();
    check_and_print(&v)
}
