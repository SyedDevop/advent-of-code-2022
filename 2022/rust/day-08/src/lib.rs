use std::{dbg, print};

fn check_row(idx: usize, matrix: &Vec<u32>) -> bool {
    let mut result: Vec<bool> = vec![];
    let mut result_left: Vec<bool> = vec![];
    for right in (idx + 1)..matrix.len() {
        if matrix[idx] >= matrix[right] {
            result.push(true);
        } else {
            result.push(false);
        }
    }
    for left in 0..idx {
        if matrix[idx] >= matrix[left] {
            result_left.push(true);
        } else {
            result_left.push(false);
        }
    }
    result.iter().all(|&x| x) || result_left.iter().all(|&x| x)
}
pub fn process_part1(input: &str) -> String {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|i| i.chars().map(|a| a.to_digit(10).unwrap()).collect())
        .collect();
    let (y, x) = (matrix.len(), matrix[0].len());
    let mut a = 0;
    for ya in 1..y {
        for xa in 1..x {
            let v = check_row(xa, &matrix[ya]);
            if v {
                a += 1;
            }
        }
    }
    a.to_string()
}

pub fn process_part2(input: &str) -> String {
    let _matrix = input;
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";
    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "21");
    }

    #[test]
    #[ignore = "Not Implemented"]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "24933642");
    }
}
