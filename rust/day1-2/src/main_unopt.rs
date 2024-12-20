use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    // Read input from file
    let input = read_lines("input.txt")?;
    
    // Parse the input into two vectors
    let (left_list, right_list) = parse_input(input);
    
    // Calculate the similarity score
    let similarity_score = calculate_similarity_score(&left_list, &right_list);
    
    // Output the result
    println!("Similarity score: {}", similarity_score);
    
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
    
    (left_list, right_list)
}

fn calculate_similarity_score(left_list: &[i32], right_list: &[i32]) -> i32 {
    // Count occurrences of each number in the right list
    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for &num in right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    left_list.iter()
        .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
        .sum()
}
