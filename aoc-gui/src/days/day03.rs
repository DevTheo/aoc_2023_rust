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

fn day03_sum_gear_ratios(data: Vec<Vec<Day03CharType>>) -> usize {
    // 
    0
}

pub fn exec() -> usize {
    let mode = 1;
    
    let input = read_input_file(FILE_NAME);

    let data = day03_parse_data(input);

    let num = if mode == 1 {
        day03_sum_part_numbers(data)
    } else {
        day03_sum_gear_ratios(data)
    }
    info!("Total {}", num);

    num
}

#[cfg(test)]
mod tests {
    use super::{day03_parse_data, day03_sum_part_numbers};

    // number positions for symbols:
    // 1 2 3 4
    // 5 6 7 A
    // B C D E

    fn exec_full_process (input: String) -> usize {
        let data = day03_parse_data(input);
        day03_sum_part_numbers(data)
    } 

    #[test]
    fn full_process_with_2_digits_pos_1_symbol_5_works() {
        let num = exec_full_process(String::from("\
          12..\n\r \
          *...\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }
    
    #[test]
    fn full_process_with_2_digits_pos_2_symbol_5_works() {
        let num = exec_full_process(String::from("\
          .12.\n\r \
          *...\n\r \
          ....\n\r"));
        assert_eq!(12, num);
    }
        
    #[test]
    fn full_process_with_2_digits_pos_3_symbol_5_works() {
        let num = exec_full_process(String::from("\
          ..12\n\r \
          *...\n\r \
          ....\n\r"));

          assert_ne!(12, num);
    }

    #[test]
    fn full_process_with_2_digits_pos_1_symbol_6_works() {
        let num = exec_full_process(String::from("\
          12..\n\r \
          .*..\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }
    
    #[test]
    fn full_process_with_2_digits_pos_2_symbol_6_works() {
        let num = exec_full_process(String::from("\
          .12.\n\r \
          .*..\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }
        
    #[test]
    fn full_process_with_2_digits_pos_3_symbol_6_works() {
        let num = exec_full_process(String::from("\
          ..12\n\r \
          .*..\n\r \
          ....\n\r"));

        assert_eq!(12, num);
    }

}