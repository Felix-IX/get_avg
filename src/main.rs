pub mod modules;
use modules::{
    helper::check_and_print, 
    input::get_input, 
    prompts::greetings,
};

fn main() {
    // GET_AVG
    // Input numbers and the program will calculate the average
    greetings();

    let v: Vec<f64> = get_input();
    check_and_print(&v)
}
