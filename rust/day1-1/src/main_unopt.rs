use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    // Read input from file
    let input = read_lines("input.txt")?;
    
    // Parse the input into two vectors
    let (mut left_list, mut right_list) = parse_input(input);
    
    // Sort both lists
    left_list.sort();
    right_list.sort();
    
    // Calculate the total distance
    let total_distance = calculate_total_distance(&left_list, &right_list);
    
    // Output the result
    println!("Total distance: {}", total_distance);
    
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn parse_input(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    // Define a regex for splitting on whitespace (tabs or spaces)
    let re = Regex::new(r"\s+").unwrap();

    for line in input {
        let parts: Vec<&str> = re.split(&line).collect();
        if parts.len() == 2 {
            if let (Ok(left_num), Ok(right_num)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
                left_list.push(left_num);
                right_list.push(right_num);
            }
        }
    }
    
    println!("Left list: {:?}", left_list);  // Debugging
    println!("Right list: {:?}", right_list); // Debugging
    
    (left_list, right_list)
}

fn calculate_total_distance(left_list: &[i32], right_list: &[i32]) -> i32 {
    left_list.iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}
