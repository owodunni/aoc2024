use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide a file as command line argument.");

    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let distance = calculate_location_list_distance(&contents);
    println!("Distance: {distance}");

    let similarity = calculate_similarity_index(&contents);
    println!("Similarity: {similarity}");
}

fn create_list(contents: &String) -> (Vec<i64>, Vec<i64>) {
    let lines = contents.lines();

    lines
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|locations| (locations[0], locations[1]))
        .unzip()
}

fn calculate_location_list_distance(contents: &String) -> i64 {
    let (mut left, mut right) = create_list(contents);
    left.sort();
    right.sort();

    let mut distance: i64 = 0;

    for it in left.iter().zip(right.iter()) {
        let (l, r) = it;
        distance = (l - r).abs() + distance
    }

    distance
}

fn calculate_similarity_index(contents: &String) -> i64 {
    let (left, right) = create_list(contents);

    let mut similarity: i64 = 0;

    for left_it in left.iter() {
        let mut found = 0;
        for rht_it in right.iter() {
            if left_it == rht_it {
                found = found + 1
            }
        }

        similarity = similarity + found * left_it;
    }

    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOCATION_LIST: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_location_list_distance() {
        let location_list = String::from(LOCATION_LIST);
        let result = calculate_location_list_distance(&location_list);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_similarity_index() {
        let location_list = String::from(LOCATION_LIST);
        let result = calculate_similarity_index(&location_list);
        assert_eq!(result, 31);
    }
}
