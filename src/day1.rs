use std::collections::HashMap;
use std::fs;
use std::process;

pub fn run() {
    let path = "./calibrations.txt";
    let calibrations = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Could not read file: {}", err.to_string());
        process::exit(1);
    });
    
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut word_digit_map : HashMap<&str, u32> = HashMap::new();
    for i in 1..10 {
        word_digit_map.insert(&words[i - 1], i.try_into().unwrap());
    }
    
    let sum: u32 = calibrations
        .split_whitespace()
        .map(|calibration| {
            let mut string_buffer = String::new();
            let mut nums = vec![];
            let mut last_char: char;
            for c in calibration.chars() {
                if c.is_numeric() {
                    nums.push(c.to_digit(10).unwrap());
                    string_buffer.clear();
                    continue;
                }            
                string_buffer.push(c);

                let does_a_word_start_with = words.iter().any(|word| word.starts_with(&string_buffer));
                if !does_a_word_start_with {
                    string_buffer.clear();
                    string_buffer.push(c);
                    continue;
                }
                
                if let Some(found_word) = words.into_iter().find(|word| word.eq(&string_buffer)) {
                    let value = word_digit_map.get(found_word).unwrap();
                    nums.push(*value);
                    last_char = string_buffer.chars().last().unwrap();
                    string_buffer.clear();
                    if words.iter().any(|word| word.starts_with(last_char)) {
                        string_buffer.push(last_char);
                    }
                    continue;
                }
            }
            
            let first_digit = nums.first().unwrap();
            let second_digit = nums.last().unwrap();
            
            format!("{first_digit}{second_digit}").parse::<u32>().unwrap()
        })
        .sum();

    println!("The answer is: {sum}");

}