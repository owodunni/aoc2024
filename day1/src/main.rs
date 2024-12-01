use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide a file as command line argument.");

    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = calculate_location_list_distance(contents);
    println!("Distance: {result}");
}

fn calculate_location_list_distance(contents: String) -> i64 {
    let lines = contents.lines();

    let (mut left, mut right): (Vec<i64>, Vec<i64>) = lines
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|locations| (locations[0], locations[1]))
        .unzip();

    left.sort();
    right.sort();

    let mut distance: i64 = 0;

    for it in left.iter().zip(right.iter()) {
        let (l, r) = it;
        distance = (l - r).abs() + distance
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const LOCATION_LIST: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = calculate_location_list_distance(String::from(LOCATION_LIST));
        assert_eq!(result, 11);
    }
}
