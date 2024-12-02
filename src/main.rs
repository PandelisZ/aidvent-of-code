use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split_whitespace().collect();
                left.push(parts[0].parse::<i32>().unwrap());
                right.push(parts[1].parse::<i32>().unwrap());
            }
        }
    }

    left.sort();
    right.sort();

    let mut total_distance = 0;

    for (l, r) in left.iter().zip(right.iter()) {
        total_distance += (l - r).abs();
    }

    println!("Total distance: {}", total_distance);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
