use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Read input from file
    let input = read_lines("input.txt")?;
    
    // Parse and evaluate reports
    let safe_count = input.iter()
        .filter(|line| is_safe_with_dampener(line))
        .count();
    
    // Output the result
    println!("Number of safe reports: {}", safe_count);
    
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

fn is_safe_with_dampener(report: &str) -> bool {
    let levels: Vec<i32> = report
        .split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    if levels.len() < 2 {
        return false; // A report with fewer than 2 levels can't be evaluated
    }

    // Check if the report is inherently safe
    if is_safe(&levels) {
        return true;
    }

    // Try removing one level at a time and check if the modified report is safe
    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        if is_safe(&modified_levels) {
            return true;
        }
    }

    false // Not safe even after removing one level
}

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let mut differences = Vec::new();
    for pair in levels.windows(2) {
        let diff = pair[1] - pair[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false; // Difference out of range
        }
        differences.push(diff);
    }

    let all_increasing = differences.iter().all(|&d| d > 0);
    let all_decreasing = differences.iter().all(|&d| d < 0);

    all_increasing || all_decreasing
}
