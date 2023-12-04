use std::fs::File;
use std::io::{self, BufRead};

mod day_1;

fn main() {
    let day_number = take_terminal_input_riddle_number();
    let input_filename = format!("input/day_{}.txt", day_number);
    // Handle the Result returned by read_riddle_input_from_file
    let riddle_input = match read_riddle_input_from_file(&input_filename) {
        Ok(content) => content,
        Err(err) => {
            println!("Error reading file: {}", err);
            return;
        }
    };

    // Call the function dynamically based on the day_number
    match day_number {
        1 => day_1::solve_day_1(&riddle_input),
        // 2 => day_2::function(),
        _ => println!("No implementation for day {}", day_number),
    }
}

fn take_terminal_input_riddle_number() -> usize {
    println!("Enter something:");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    match user_input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Invalid input. Using default value 1.");
            1 // Provide a default value or handle the error accordingly
        }
    }
}

fn read_riddle_input_from_file(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut content = String::new();

    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n'); // Add a newline to separate lines, adjust as needed
    }
    Ok(content)
}