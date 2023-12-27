use std::collections::HashMap;

use crate::days::common::read_input_file;

const FILE_NAME: &str = "./input/day02/puzzle-input1.txt";

//#[derive(Debug)]
struct GameData {
    game: i32,
    data: HashMap<String, i32> 
}

impl GameData {
    fn new(game: i32) -> GameData {
        GameData {
             game,
             data: HashMap::new()
        }   
    }
    fn add_color(&mut self, color: &str, count: &mut i32) {
        let current = self.data.entry(String::from(color)).or_insert(*count);
        if current < count {
            *current = *count;
        }
    }
}

fn parse_line(line: &str) -> GameData {
    // format is 
    // Game #: {color_name} [count], {color_name} [count]...; {color_name} [count], {color_name} [count]... 
    info!("parsing line {}", line);
    let line_data: Vec<&str> = line.split(':').collect();
    //info!("   line data {}- {}", line_data[0], line_data[1]);
    let game_num_data: Vec<&str> = line_data[0].split(' ').collect();
    //info!("   game num data {}- {}", game_num_data[0], game_num_data[1]);
    let all_game_data: Vec<&str> = line_data[1].split(';').collect();
    //info!("   all game data {}", line_data[1]);
    
    //info!("   game_num {}", game_num_data[1]);
    let game_num: i32 = game_num_data[1].parse::<i32>().unwrap();

    let mut result = GameData::new(game_num);

    for game_line in &all_game_data {
        let color_info: Vec<&str> = game_line.trim().split(',').collect();
        for color_data in &color_info {
            //info!("   color info {}", color_data);
            let key_value: Vec<&str> = color_data.trim().split(' ').collect();
            //info!("   color kvp {}- {}", key_value[0], key_value[1]);
            let color_name = key_value[1].trim();
            let mut color_count = key_value[0].trim().parse::<i32>().unwrap();
            result.add_color(color_name, &mut color_count);
        }
    } 

    result
}

fn parse_data(lines: String) -> Vec<GameData> {
    let mut result: Vec<GameData> = Vec::new();
    let line_data: Vec<&str> = lines.split('\n').collect();
    for line in line_data {
        result.push(parse_line(line.trim()))
    }

    result
}

pub fn exec() -> i32 {
    let aoc_part = 2; // advent of code part 
    let red_count = 12;
    let green_count = 13;
    let blue_count = 14;

    let input = read_input_file(FILE_NAME);

    let game_data = parse_data(input);

    let mut solution = 0;

    for item in game_data {
        let game_num = item.game;
        let red = item.data["red"];
        let blue = item.data["blue"];
        let green = item.data["green"];
        
        if aoc_part != 2 {
            //info!("Game {}: red {}, green {}, blue {}", game_num, red, green, blue);
            if red <= red_count && blue <= blue_count && green <= green_count {
                solution += game_num;
            } 
        } else {
            let pow = red * green * blue;
            info!("Game {}: power {}", game_num, pow);
            
            solution += pow;            
        }
    }

    info!("solution: {}", solution);
    solution
}
