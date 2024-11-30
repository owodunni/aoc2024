use regex::Regex;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("Input file: {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = sum_engine_parts(contents);

    println!("Engine sum: {}", result)
}

fn sum_engine_parts(schematics: String) -> i64 {
    dbg!(schematics);
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let hay = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";

    let mut dates = vec![];
    for (_, [year, month, day]) in re.captures_iter(hay).map(|c| c.extract()) {
        dates.push((year, month, day));
    }
    assert_eq!(dates, vec![
      ("2010", "03", "14"),
      ("2014", "10", "14"),
    ]);;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const ENGINE_PARTS: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = sum_engine_parts(String::from(ENGINE_PARTS));
        assert_eq!(result, 0);
    }
}
