use std::collections::HashMap;
use std::fs;
use std::process;

fn main() {
    let path = "./calibrations.txt";
    let calibrations = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Could not read file: {}", err.to_string());
        process::exit(1);
    });
    
    let word_digit_map : HashMap<&str, usize> = HashMap::from([
      ("one", 1),
      ("two", 2),
      ("three", 3),
      ("four", 4),
      ("five", 5),
      ("six", 6),
      ("seven", 7),
      ("eight", 8),
      ("nine", 9),
    ]);
    let words: Vec<&&str> = word_digit_map.keys().collect();
    
    let sum: u32 = calibrations
        .split_whitespace()
        .map(|calibration| {
            let nums = calibration
                .chars()
                .filter(|c| {
                    c.is_numeric()
                })
                .collect::<Vec<_>>();
                

            let first_digit = nums.first().unwrap();
            let second_digit = nums.last().unwrap();
            
            format!("{first_digit}{second_digit}").parse::<u32>().unwrap()
        })
        .sum();

    println!("The answer is: {sum}");
}
