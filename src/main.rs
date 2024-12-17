use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sum: i32 = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Parse error"))
        .sum();
    println!("{}", sum);
}
