use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide a file as command line argument.");

    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let reports = create_report(&contents);

    let safety = is_reports_safe(&reports);

    println!("Safety rating: {safety}");

    let safety = is_reports_safe_dampner(&reports);

    println!("Safety rating dampner: {safety}");
}

fn create_report(contents: &String) -> Vec<Vec<i64>> {
    let lines = contents.lines();

    lines
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn is_reports_safe(reports: &Vec<Vec<i64>>) -> i64 {
    reports.iter().map(|report| is_safe(report)).sum()
}

fn is_reports_safe_dampner(reports: &Vec<Vec<i64>>) -> i64 {
    reports.iter().map(|report| is_safe_dampner(report)).sum()
}

fn is_safe_dampner(report: &Vec<i64>) -> i64 {
    if is_safe(report) == 1 {
        return 1;
    }
    for (i, _) in report.iter().enumerate() {
        let mut r = report.clone();
        r.remove(i);

        if is_safe(&r) == 1 {
            return 1;
        }
    }
    return 0;
}

fn is_safe(report: &Vec<i64>) -> i64 {
    // Increasing 1, decreasing -1
    let mut direction = 0;

    for (i, level) in report.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let previous = report[i - 1];

        if direction == 0 {
            if previous > *level {
                direction = 1;
            } else {
                direction = -1;
            }
        }

        let diff = direction * (previous - level);

        // The direction is the same
        if diff < 0 {
            return 0;
        }

        if diff < 1 || diff > 3 {
            return 0;
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOCATION_LIST: &'static str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_is_report_safe() {
        let report: Vec<i64> = vec![7, 6, 4, 2, 1];
        let result = is_safe(&report);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_is_report_safe_dampner() {
        let report: Vec<i64> = vec![1, 3, 2, 4, 5];
        let result = is_safe_dampner(&report);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_is_reports_safe() {
        let reports_list = String::from(LOCATION_LIST);
        let reports = create_report(&reports_list);
        assert_eq!(is_reports_safe(&reports), 2);
    }
}
