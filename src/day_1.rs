pub fn solve_day_1(riddle_input: &str) {
    // Split the input into lines
    let lines: Vec<&str> = riddle_input.lines().collect();

    // Initialize variables to accumulate numbers
    let mut total: u32 = 0;

    // Iterate through each line
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        // Find the first numeric character
        for char in line.chars() {
            if char.is_numeric() {
                first_digit = Some(char.to_digit(10).unwrap());
                break;
            }
        }

        // Reverse the line manually
        let reversed_line: String = line.chars().rev().collect();

        // Find the last numeric character in the reversed line
        for char in reversed_line.chars() {
            if char.is_numeric() {
                last_digit = Some(char.to_digit(10).unwrap());
                break;
            }
        }

        // Add the digits to the total if they are present
        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            total += first * 10 + last;
        }
    }

    println!("Result: {}", total);
}