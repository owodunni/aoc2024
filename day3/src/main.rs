use std::{env, fs};

use regex::Regex;

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide a file as command line argument.");

    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let instructions = read_instructions(&contents);

    let multiple_result = multiply_instructions(&instructions);

    println!("Multiply result: {multiple_result}");
}

fn read_instructions(contents: &String) -> Vec<(i16, i16)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re2 = Regex::new(r"don't\(\).*?(?:do\(\)|$)").unwrap();
    contents
        .lines()
        .map(|line| {
            println!("line: {line}");
            let new_line = re2.replace_all(line, "");
            println!("line: {new_line}");
            new_line
        })
        .flat_map(|line| {
            //println!("line: {line}");
            re.captures_iter(&*line)
                .map(|cap| {
                    //print!("cap: {}, {}, {}", &cap[0], &cap[1], &cap[2]);
                    let res = (
                        cap[1].parse::<i16>().unwrap(),
                        cap[2].parse::<i16>().unwrap(),
                    );

                    //print!("result: ({}), ({})", &res.0, &res.1);

                    res
                })
                .collect::<Vec<(i16, i16)>>()
        })
        .collect()
}

fn multiply_instructions(instructions: &Vec<(i16, i16)>) -> i32 {
    instructions
        .iter()
        .map(|i| (i.0 as i32) * (i.1 as i32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_read_instructions() {
        let instructions = String::from(INPUT);
        let result = read_instructions(&instructions);
        assert_eq!(result, vec![(2, 4), (5, 5), (11, 8), (8, 5)]);

        let res = multiply_instructions(&result);
        assert_eq!(res, 48);
    }
}
