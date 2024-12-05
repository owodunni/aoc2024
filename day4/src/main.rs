use std::{env, fs, ops};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("Please provide a file as command line argument.");

    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let grid = create_grid(&contents);

    let count = find_xmas(&grid, 'X');

    print!("Count: {count}");

    let count = find_mas(&grid, 'A');

    print!("Count: {count}")
}

type Grid = Vec<Vec<char>>;
#[derive(Debug)]
struct P {
    x: i16,
    y: i16,
}

fn create_grid(contents: &String) -> Grid {
    let lines = contents.lines();

    lines.map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &Grid, start: char) -> Vec<P> {
    let mut points = vec![];

    for (y, line) in grid.iter().enumerate() {
        for (x, v) in line.iter().enumerate() {
            if start == *v {
                points.push(P {
                    x: x as i16,
                    y: y as i16,
                })
            }
        }
    }
    return points;
}

fn find_xmas(grid: &Grid, start: char) -> u32 {
    let start = find_start(grid, start);

    let max_y = grid.len() as i16;
    let max_x = grid[0].len() as i16;

    let mut instances = 0;

    let directions = vec![
        P { x: -1, y: -1 },
        P { x: -1, y: 0 },
        P { x: -1, y: 1 },
        P { x: 0, y: -1 },
        P { x: 0, y: 1 },
        P { x: 1, y: -1 },
        P { x: 1, y: 0 },
        P { x: 1, y: 1 },
    ];

    let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];

    for p in start.iter() {
        for dir in directions.iter() {
            for (i, curr_char) in xmas.iter().enumerate() {
                let y = p.y + (dir.y * (i as i16));
                let x = p.x + (dir.x * (i as i16));
                if y < 0 || x < 0 || y >= max_y || x >= max_x {
                    break;
                }

                if grid[y as usize][x as usize] != *curr_char {
                    break;
                }

                if i == xmas.len() - 1 {
                    instances += 1;
                }
            }
        }
    }
    instances
}

fn find_mas(grid: &Grid, start: char) -> u32 {
    let start = find_start(grid, start);
    //dbg!(&start);

    let max_y = grid.len() as i16;
    let max_x = grid[0].len() as i16;

    let max = P { x: max_x, y: max_y };

    let mut instances = 0;

    let directions = vec![
        P { x: -1, y: -1 },
        P { x: -1, y: 1 },
        P { x: 1, y: -1 },
        P { x: 1, y: 1 },
    ];

    let xmas: Vec<char> = vec!['M', 'A', 'S'];

    for p in start.iter() {
        let mut matches = 0;
        for dir in directions.iter() {
            let corner = P {
                y: p.y + dir.y,
                x: p.x + dir.x,
            };
            let rev_dir = P {
                y: -dir.y,
                x: -dir.x,
            };
            matches += is_word(grid, &corner, &rev_dir, &xmas, &max);
        }

        if matches == 2 {
            instances += 1;
        }
    }
    instances
}

fn is_word(grid: &Grid, p: &P, dir: &P, word: &Vec<char>, max: &P) -> u32 {
    //println!("p:({}, {}) dir:({}, {})", p.x, p.y, dir.x, dir.y);
    for (i, curr_char) in word.iter().enumerate() {
        let y = p.y + (dir.y * (i as i16));
        let x = p.x + (dir.x * (i as i16));
        //println!("next:({x}, {y}), i:{i}");
        if y < 0 || x < 0 || y >= max.y || x >= max.x {
            break;
        }

        if grid[y as usize][x as usize] != *curr_char {
            break;
        }

        if i == word.len() - 1 {
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORD_LIST: &'static str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const WORD_LIST_SMALL: &'static str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....";

    #[test]
    fn test_is_create_grid() {
        let words = String::from(WORD_LIST);
        let grid = create_grid(&words);
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
        assert_eq!(grid[9].len(), 10);

        let start = find_start(&grid, 'X');

        assert_eq!(start.len(), 19);
    }

    #[test]
    fn test_find_xmas() {
        let words = String::from(WORD_LIST_SMALL);
        let grid = create_grid(&words);

        let xmas = find_mas(&grid, 'A');

        assert_eq!(xmas, 5);
    }

    #[test]
    fn test_find_mas() {
        let words = String::from(WORD_LIST);
        let grid = create_grid(&words);

        let xmas = find_mas(&grid, 'A');

        assert_eq!(xmas, 9);
    }
}
