
# Simple Average Calculator in Rust

## Overview

This Rust program provides a command-line interface for calculating the average of a series of floating-point numbers entered by the user. The user can enter multiple numbers and choose to stop inputting by entering the character "z". The application gracefully handles invalid inputs and calculates the average when at least one valid number has been entered.

### Key Features

- **User Input**: Prompts the user to input numbers with an ordinal suffix for each entry.
- **Number Validation**: Validates user input, allowing only valid floating-point numbers or the quit signal 'z'.
- **Average Calculation**: Calculates the average of all valid numbers entered by the user.
- **Graceful Termination**: The program stops accepting new numbers when the user enters 'z'.

## Functions Summary

1. `main()`: Initiates the program, prompts the user for input. 
2. `read_number(input_line: &str, input_vec: &mut Vec<f64>) -> (bool, bool)`: A helper function that reads a line of input from the user and returns a tuple indicating whether the input was a valid number (`bool 1`) or if the user chose to quit (`bool 2`), updating the input vector accordingly.
3. `get_input() -> Vec<f64>`: This function repeatedly prompts the user for input until they indicate they want to stop by entering 'z'. It returns a vector containing all the valid numbers entered.
4. `cal_avg(v: &Vec<f64>) -> f64`: A helper function that calculates and returns the average of the provided vector of floating-point numbers.
5. `check_and_print(v: &Vec<f64>)`: A helper function that checks if the vector contains at least one valid number, and if so, prints the average of the provided vector of floating-point numbers.
6. `ordinal_suffix(num: usize) -> &'static str`: A utility function used to append the correct ordinal suffix to the prompt based on the current iteration count.

## Usage

To run the program, compile it using the Rust compiler:
```bash
$ cargo run
```
The program will start prompting for input numbers, and upon receiving enough valid input or encountering the 'z' signal, it will output the average of the entered numbers.

## Contributing

If you find any issues or have suggestions for improvement, feel free to open an issue or submit a pull request. This code is designed to be modular, readable, and maintainable, adhering to Rust's best practices and principles.

Enjoy using this simple yet functional average calculator!

**Note**: The provided Rust code snippet demonstrates good programming practices such as error handling, modularity, and clear comments which enhance its readability and maintainability.

## License

This project is licensed under the GNU General Public License version 3.0 (GPLv3). By using, distributing, or modifying this software, you agree to be bound by the terms of the GPLv3. You can find a copy of the license in the LICENSE file within the repository.

Please note that any contributions made to this project are also subject to the GPL 3.0 License.
