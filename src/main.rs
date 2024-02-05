use get_avg::helper::check_and_print;
pub use get_avg::{helper::cal_avg, input::get_input};

fn main() {
    println!("Input numbers and the program will calculate the average");

    let v: Vec<f64> = get_input();
    check_and_print(&v)
}
