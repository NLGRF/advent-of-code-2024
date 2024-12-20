use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use once_cell::sync::Lazy;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());

fn main() -> io::Result<()> {
    // อ่านข้อมูลจากไฟล์
    let input = read_lines("input.txt")?;
    
    // แยกข้อมูลเป็นสองลิสต์
    let (left_list, right_list) = parse_input(input);
    
    // คำนวณคะแนนความคล้ายคลึง
    let similarity_score = calculate_similarity_score(&left_list, &right_list);
    
    // แสดงผลลัพธ์
    println!("Similarity score Opt: {}", similarity_score);
    
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
    let mut left_list = Vec::with_capacity(input.len());
    let mut right_list = Vec::with_capacity(input.len());
    
    for line in input {
        let parts: Vec<&str> = RE.split(&line).collect();
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
    let right_counts = right_list.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    left_list.iter()
        .filter_map(|&num| right_counts.get(&num).map(|&count| num * count))
        .sum()
}
