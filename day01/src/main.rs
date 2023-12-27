use std::fs::File;
use std::io::prelude::*;

const FILE_NAME: &str = "./puzzle-input2.txt";
fn main() ->  std::io::Result<()> {
    let mut file = File::open(FILE_NAME)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let total = day01_calc(&contents);
    println!("total = {}", total);
    Ok(())
}


fn day01_calc(input: &String) -> usize {
    let lines = input.split('\n');
    let digits = "0123456789".as_bytes();
    let mut total = 0;
    for (_, line) in lines.enumerate() {
        let mut num: [usize; 2] = [0,0];
        let mut idx = 0;
        let fixed = day01_fix_spelled_numbers(String::from(line.trim()));
        let bytes = fixed.as_bytes();
        for (_, ch) in bytes.iter().enumerate() { 
            let usz = digits.iter().position(|&r| r == *ch);
            if let Some(i) = usz {
                num[idx] = i;
                if idx < 1 {
                    idx += 1;
                    num[1] = i;
                }
            }
        }
        total += (num[0] * 10) + num[1];
        println!("{} ({}) = {}", line.trim(), fixed, (num[0] * 10) + num[1]);
    }
    total
}

fn day01_fix_spelled_numbers(line: String) -> String {
    line.to_lowercase()
        .replace("zero",  "zero0zero")
        .replace("one",   "one1one") 
        .replace("two",   "two2two") 
        .replace("three", "three3three") 
        .replace("four",  "four4four") 
        .replace("five",  "five5five") 
        .replace("six",   "six6six") 
        .replace("seven", "seven7seven") 
        .replace("eight", "eight8eight") 
        .replace("nine",  "nine9nine") 
}