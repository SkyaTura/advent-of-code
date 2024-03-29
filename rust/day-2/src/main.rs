#![allow(unused, dead_code)]
use std::io;

fn main() {
    let mut result = 0;
    for line in io::stdin().lines() {
        let line_result = match line.unwrap().as_ref() {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        };
        result += line_result;
    }
    println!("Result: {}", result);
}
