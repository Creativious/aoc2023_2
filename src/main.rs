use std::fs::File;
use std::io;
use std::io::BufRead;
use regex::Regex;

#[derive(PartialEq)]
enum Color {
    Blue,
    Green,
    Red
}

struct ColorCount {
    color: Color,
    count: i32
}

impl ColorCount {
    fn new(color_c: Color, color_count: i32) -> Self {
        ColorCount {
            color: color_c,
            count: color_count,
        }
    }
}

fn main() {
    let filename = "puzzle_input.txt";
    let file_lines = read_lines(filename);
    let mut total = 0;
    let max_red_cubes = 12;
    let max_green_cubes = 13;
    let max_blue_cubes = 14;
    for line in file_lines {
        let game_id_string_prefix = get_string_with_regex_single(&*line, Regex::new(r"\d*:").unwrap());
        let game_id_string = remove_last_char_in_string(game_id_string_prefix);
        let game_id: i32 = game_id_string.parse().unwrap();
        let shows = get_string_with_regex_multiple(&*line, Regex::new(r"(?m)(\s\d*\s[^;]*)").unwrap());
        let mut is_game_impossible = false;
        for show in shows {
            let color_counts = get_colorcounts_from_regex(&*show, Regex::new(r"(?m)(\d*\S)\s(\w*[^,]),?").unwrap());
            let mut red_count = 0;
            let mut green_count = 0;
            let mut blue_count = 0;
            for color_count in color_counts {
                if (color_count.color == Color::Blue) {
                    blue_count = color_count.count;
                    if (blue_count > max_blue_cubes) {
                        is_game_impossible = true;
                        break;
                    }
                }
                else if (color_count.color == Color::Green) {
                    green_count = color_count.count;
                    if (green_count > max_green_cubes) {
                        is_game_impossible = true;
                    }
                }
                else if (color_count.color == Color::Red) {
                    red_count = color_count.count;
                    if (red_count > max_red_cubes) {
                        is_game_impossible = true;
                    }
                }
            }
            if (is_game_impossible) {
                break;
            }

        }
        if (!is_game_impossible) {
            total += game_id;
            println!("Possible Game: {}", game_id)
        } else {
            println!("Impossible Game: {}", game_id)
        }

    }
    print!("{}", total);
}

fn remove_last_char_in_string(string: String) -> String {
    let mut chars = string.chars();
    chars.next_back();
    chars.as_str().to_string()
}
fn remove_first_char_in_string(string: String) -> String {
    let mut chars = string.chars();
    chars.next();
    chars.as_str().to_string()
}
fn get_string_with_regex_single(line: &str, re: Regex) -> String {
    let results = re.captures_iter(&*line);
    let mut temp = "".to_string();
    for mat in results {
        temp = mat.iter().next().unwrap().unwrap().as_str().to_string();

    }
    return temp;
}
fn get_string_with_regex_multiple(line: &str, re: Regex) -> Vec<String> {
    let results = re.captures_iter(&*line);
    let mut matches = Vec::new();
    for mat in results {
        matches.push(mat.iter().next().unwrap().unwrap().as_str().to_string());
    }
    matches
}

fn get_colorcounts_from_regex(line: &str, re: Regex) -> Vec<ColorCount>{
    let results = re.captures_iter(&*line);
    let mut color_counts = Vec::new();
    for mat in results {
        let count: i32 = mat.get(1).unwrap().as_str().to_string().parse().unwrap();
        let color_string = mat.get(1).unwrap().as_str().to_string();
        let color = match color_string.as_str() {
            "blue" => {
                Color::Blue
            }
            "red" => {
                Color::Red
            }
            "green" => {
                Color::Green
            }
            _ => {
                Color::Green
            }
        };
        let color_count = ColorCount::new(color, count);
        color_counts.push(color_count);
    }
    color_counts
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    let lines = io::BufReader::new(file.unwrap()).lines();
    let mut lines_in_string = Vec::new();
    for line in lines {
        lines_in_string.push(line.unwrap());
    }
    lines_in_string
}