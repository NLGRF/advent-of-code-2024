use regex::Regex;
use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read the file.");

    // Regular expression to match valid mul(X,Y) patterns
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Variable to store the total sum of results
    let mut total_sum = 0;

    // Find all matches in the input
    for cap in re.captures_iter(&input) {
        // Parse the numbers from the capture groups
        let x: i32 = cap[1].parse().expect("Failed to parse X.");
        let y: i32 = cap[2].parse().expect("Failed to parse Y.");

        // Multiply the numbers and add to the total sum
        total_sum += x * y;
    }

    // Print the total sum
    println!("The total sum of all valid multiplications is: {}", total_sum);
}
