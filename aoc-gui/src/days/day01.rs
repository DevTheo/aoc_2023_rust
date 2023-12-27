use crate::days::common::{read_input_file, eval_digit};

const FILE_NAME: &str = "./input/day01/puzzle-input2.txt";

pub fn exec() -> String {

    let contents = read_input_file(FILE_NAME);
    let total = day01_calc(&contents);

    info!("Day01: {}", total);
    format!("{}", total)
}    

fn day01_calc(input: &String) -> usize {
    let lines = input.split('\n');
    let mut total = 0;
    for (_, line) in lines.enumerate() {
        let mut num: [usize; 2] = [0,0];
        let mut idx = 0;
        let fixed = day01_fix_spelled_numbers(String::from(line.trim()));
        let bytes = fixed.as_bytes();
        for (_, ch) in bytes.iter().enumerate() { 
            let usz = eval_digit(*ch);
            if let Some(i) = usz {
                num[idx] = i;
                if idx < 1 {
                    idx += 1;
                    num[1] = i;
                }
            }
        }
        total += (num[0] * 10) + num[1];
        info!("{} ({}) = {}", line.trim(), fixed, (num[0] * 10) + num[1]);
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


