use std::collections::HashMap;

use crate::days::common::read_input_file;

use super::common::eval_digit;

const FILE_NAME: &str = "./input/day03/puzzle-input1.txt";

#[derive(Copy, Clone)]
enum Day03CharType {
    Digit(usize),
    Symbol(u8),
    Empty
}


fn day03_load_line(line: &str) -> Vec<Day03CharType> {
    let mut result: Vec<Day03CharType> = Vec::new();
    
    // pad with empty column up front
    //result.push(Day03CharType::Empty);
    
    let line_bytes = line.as_bytes();
    for byte in line_bytes {        
        let maybe_digit = eval_digit(*byte);
        match maybe_digit {
            Some(digit) => result.push(Day03CharType::Digit(digit)),
            None => {
                if *byte != b'.' {
                    result.push(Day03CharType::Symbol(*byte));
                } else {
                    result.push(Day03CharType::Empty);
                }
            } 
        }
    }
    // to avoid an issue with our algorthm, we'll just add an extra empty column at the end
    result.push(Day03CharType::Empty);
    
    result
}

fn day03_parse_data(lines: String) -> Vec<Vec<Day03CharType>> {
    let mut result: Vec<Vec<Day03CharType>> = Vec::new();
    let line_data: Vec<&str> = lines.split('\n').collect();
    for line in line_data {
        result.push(day03_load_line(line.trim()))
    }
    result
}


fn has_adjacent_symbol(data: &Vec<Vec<Day03CharType>>, line_num: usize, start: usize, end: usize) -> bool {
    let line_len = data[line_num].len();
    
    let left = if start > 0 { start - 1 } else { start };
    let right = if end < line_len { end + 1 } else { end };
    let top = if line_num > 0 { line_num - 1 } else { line_num };
    let bottom = if line_num < data.len()-1 { line_num + 1 } else { line_num };

    //info!(" Bounds x: {}, {}  y: {}, {}", left, right, top, bottom);

    for y in top..(bottom+1) {
        for x in left..(right+1) {
            //info!("   Bound check ({}, {})", x, y);
            if let Day03CharType::Symbol(byte) = data[y][x] {
                return true;
            }             
        } 
    }

    false
}

fn day03_sum_part_numbers(data: Vec<Vec<Day03CharType>>) -> usize {
    let line_count = data.len();
    let mut result = 0; 

    for line_num in 0..line_count {
        //info!("checking line {}", (line_num + 1));
        
        if let Some(ln) = data.get(line_num) {        
            let line = ln;    
            
            let mut i = 0;
            let mut num: usize; 
            while i < line.len() {
                num = 0;
                if let Day03CharType::Digit(digit_num) = line[i] {
                    num = digit_num;
                    let mut x = 1;
                    while i + x < line.len() {
                        if let Day03CharType::Digit(digit_num) = line[i + x] {

                            num = (num * 10) + digit_num;
                            x += 1;
                            continue;
                        }
                        //info!("Checking {}", num);
                        if has_adjacent_symbol(&data, line_num, i, i+x-1) { 
                            //info!("Found {}", num);
                            result += num;
                        } else {
                            //info!("NOT Found {}", num);
                        }
                        i = i+x-1;
                        break;
                    }
                }
                // We really should be checking to see if we still have a number, but we added an extra empty column at the end to help us

                i+=1;
            }
        }  
    }
    result
}

fn day03_find_number_and_start(line: &Vec<Day03CharType>, column_num: usize) -> (usize, usize) {
    println!("day03_find_number_and_start {column_num}");
    let mut start = column_num;
    let line_len = line.len();
    while start > 0 {
        let column_data = line.get(start -1).unwrap();

        if let Day03CharType::Digit(_) = *column_data {
            start = start -1;
        } else {
            break;
        }
    }
    println!("day03_find_number_and_start start {start}");
    
    let mut num = 0;
    let mut i = 0;
    while start + i < line_len {
        let column_data = line.get(start + i).unwrap();
        if let Day03CharType::Digit(digit_num) = column_data {
            num = (num * 10) + digit_num;
        } else {
            break;
        }
        i += 1;
    }
    println!("day03_find_number_and_start num {num}");

    (start, num)
}

fn day03_get_ratio_for_gear(data: &Vec<Vec<Day03CharType>>, line_num: usize, idx: usize) -> usize {
    let line_count = data.len();
    let line_len = data.get(0).unwrap().len();

    let left = if idx > 0 { idx - 1 } else { idx };
    let right = if idx < line_len { idx + 1 } else { idx };
    let top = if line_num > 0 { line_num - 1 } else { line_num };
    let bottom = if line_num < line_count-1 { line_num + 1 } else { line_num };

    let mut nums : HashMap<String, usize> = HashMap::new();

    for y in top..(bottom+1) {
        let line = data.get(y).unwrap();

        for x in left..(right+1) {
            let column_data = line.get(x).unwrap();
            if let Day03CharType::Digit(_num) = column_data {
                let (start, num) = day03_find_number_and_start(line, x);
                
                nums.insert(format!("{y} {start}"), num);
            }
        } 
    }

    if nums.len() == 2 {
        let mut result = 1;
        for (_start, value) in nums.iter() {
            result = result * value;
        }
        return result;
    }

    // the numbers adjacent to the symbol and if there are just 2 of them
    // then multiply the numbers together and sum up those numbers
    0
}

fn day03_sum_gear_ratios(data: &Vec<Vec<Day03CharType>>) -> usize {
    let mut result = 0;
    let line_count = data.len();

    // scan through the data line by line 
    for line_num in 0..line_count {
        let line = data.get(line_num).unwrap();

        // scan each character
        let mut i = 0;
        for char in line {
            
            // looking for Symbols that are the "*" character
            if let Day03CharType::Symbol(chr) = char {
                if *chr == b'*' {
                    result += day03_get_ratio_for_gear(data, line_num, i);
                }
            }

            i += 1;
        }
    }

    // return the sum
    result
}

pub fn exec() -> usize {
    let mode = 2;
    
    let input = read_input_file(FILE_NAME);

    let data = day03_parse_data(input);

    let num = if mode == 1 {
        day03_sum_part_numbers(data)
    } else {
        day03_sum_gear_ratios(&data)
    };

    info!("Total {}", num);

    num
}

#[cfg(test)]
mod tests {
    use super::{day03_parse_data, day03_sum_part_numbers, day03_get_ratio_for_gear, day03_sum_gear_ratios};

    // number positions for symbols:
    // 1 2 3 4
    // 5 6 7 A
    // B C D E

    fn exec_full_process_part1 (input: String) -> usize {
        let data = day03_parse_data(input);
        day03_sum_part_numbers(data)
    } 

    fn exec_full_process_part2 (input: String) -> usize {
        let data = day03_parse_data(input);
        day03_sum_gear_ratios(&data)
    } 

    #[test]
    fn full_process_with_2_digits_pos_1_symbol_5_works() {
        let num = exec_full_process_part1(String::from("\
          12..\n\r \
          *...\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }
    
    #[test]
    fn full_process_with_2_digits_pos_2_symbol_5_works() {
        let num = exec_full_process_part1(String::from("\
          .12.\n\r \
          *...\n\r \
          ....\n\r"));
        assert_eq!(12, num);
    }
        
    #[test]
    fn full_process_with_2_digits_pos_3_symbol_5_works() {
        let num = exec_full_process_part1(String::from("\
          ..12\n\r \
          *...\n\r \
          ....\n\r"));

          assert_ne!(12, num);
    }

    #[test]
    fn full_process_with_2_digits_pos_1_symbol_6_works() {
        let num = exec_full_process_part1(String::from("\
          12..\n\r \
          .*..\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }
    
    #[test]
    fn full_process_with_2_digits_pos_2_symbol_6_works() {
        let num = exec_full_process_part1(String::from("\
          .12.\n\r \
          .*..\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }
        
    #[test]
    fn full_process_with_2_digits_pos_3_symbol_6_works() {
        let num = exec_full_process_part1(String::from("\
          ..12\n\r \
          .*..\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }

    // Gear ratio positions

    // . 1 2 3 .
    // . 4 * 5 .
    // . 6 7 8 .
    
    #[test]
    fn gear_ratio_pos_is_1_and_3() {
        let num = exec_full_process_part2(String::from("\
          10.10\n\r \
          ..*..\n\r \
          .....\n\r"));

        assert_eq!(100, num);
    }

    #[test]
    fn gear_ratio_pos_is_1_and_4() {
        let num = exec_full_process_part2(String::from("\
          10...\n\r \
          10*..\n\r \
          .....\n\r"));

        assert_eq!(100, num);
    }
}