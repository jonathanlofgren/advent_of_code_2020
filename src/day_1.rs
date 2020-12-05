use std::collections::HashSet;
use crate::utils;


fn solve(set: HashSet<i64>, sum: i64) -> i64 {
    for num in &set {
        let complement = sum - num;
        
        if set.contains(&complement) {
            return num * complement;
        }
    };

    -1
}

fn solve_three(set: HashSet<i64>, sum: i64) -> i64 {
    for num in &set {
        let complement = sum - num;
        let mut new_set = set.clone();
        new_set.remove(num);
        let two = solve(new_set, complement);

        if two > 0 {
            return two * num
        }
    };

    -1
}

pub fn main() {
    let set: HashSet<i64> = utils::read_lines_to_vec("data/day_1.txt").into_iter().collect();
    let set_2 = set.clone();
    println!("======== Day 1 ========");
    println!("Part 1 = {}", solve(set, 2020));
    println!("Part 2 = {}", solve_three(set_2, 2020));
}


#[test]
fn test_example_report() {
    let numbers: HashSet<i64> = vec![1721, 979, 366, 299, 675, 1456].into_iter().collect();
    assert_eq!(solve(numbers, 2020), 514579)
}

#[test]
fn test_no_solution() {
    let numbers: HashSet<i64> = vec![1, 2, 100].into_iter().collect();
    assert_eq!(solve(numbers, 100), -1)
}

#[test]
fn test_example_report_three() {
    let numbers: HashSet<i64> = vec![1721, 979, 366, 299, 675, 1456].into_iter().collect();
    assert_eq!(solve_three(numbers, 2020), 241861950)
}