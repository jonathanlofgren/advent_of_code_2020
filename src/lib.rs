use std::collections::HashSet;


pub mod day_3 {
    use super::*;

    type Grid = Vec<Vec<char>>;

    pub fn count_trees(grid: &Grid, right: usize, down: usize) -> usize {
        get_path(grid, right, down)
            .iter()
            .filter(|&c| c == &'#')
            .count()
    }

    pub fn trees_multiplied(grid: &Grid, slopes: &Vec<(usize, usize)>) -> i64 {
        slopes.iter()
            .map(|(right, down)| count_trees(grid, *right, *down) as i64)
            .product()
    }

    pub fn get_path(grid: &Grid, right: usize, down: usize) -> Vec<char> {
        let coords = (0..).step_by(right).zip((0..).step_by(down));
        let rows = grid.len();
        let cols = grid[0].len();
    
        coords
            .map(|coord| {
                if coord.1 < rows {
                    Some(grid[coord.1][coord.0 % cols])
                } else {
                    None
                }
            })
            .take_while(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }

    pub fn main() {
        let lines: Vec<String> = utils::read_lines_to_vec("data/day_3.txt");
        let grid: Grid = lines.iter().map(|l| l.chars().collect()).collect();
        let part_2_slopes: Vec<(usize, usize)> = vec![
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2),
        ];

        println!("======== Day 3 ========");
        println!("Part 1 = {}", count_trees(&grid, 3, 1));
        println!("Part 2 = {}", trees_multiplied(&grid, &part_2_slopes));
    }


    #[test]
    fn test_get_path() {
        let grid: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '.'],
        ];
        let expected = vec!['.', '#', '.', '.', '#'];

        let path = get_path(&grid, 1, 1);

        assert_eq!(path, expected);
    }

    #[test]
    fn test_trees_multiplied() {
        let grid: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '.'],
        ];
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (1, 1), (1, 1)];

        assert_eq!(trees_multiplied(&grid, &slopes), 8);
    }
}


pub mod day_2 {
    use super::*;
    use regex::Regex;

    pub type Password = (usize, usize, char, String);

    pub fn get_valid(passwords: &Vec<String>) -> Vec<Password> {
        passwords.iter().map(|x| parse_password(x)).filter(|x| is_valid(x)).collect()
    }

    pub fn is_valid(password: &Password) -> bool {
        let matches = password.3.matches(password.2).count();
        password.0 <= matches && matches <= password.1
    }

    pub fn get_valid_part_2(passwords: &Vec<String>) -> Vec<Password> {
        passwords.iter().map(|x| parse_password(x)).filter(|x| is_valid_part_2(x)).collect()
    }

    pub fn is_valid_part_2(password: &Password) -> bool {
        let chars: Vec<char> = password.3.chars().collect();
        (chars[password.0 - 1] == password.2) ^ (chars[password.1 - 1] == password.2)
    }

    pub fn parse_password(password: &String) -> Password {
        let regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
        let captures = regex.captures(&password).unwrap();

        (
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
            captures.get(3).unwrap().as_str().chars().next().unwrap(),
            captures.get(4).unwrap().as_str().to_string()
        )
    }

    pub fn main() {
        let passwords: Vec<String> = utils::read_lines_to_vec("data/day_2.txt");

        println!("======== Day 2 ========");
        println!("Part 1 = {}", get_valid(&passwords).len());
        println!("Part 1 = {}", get_valid_part_2(&passwords).len());
    }

    #[test]
    fn test_get_valid() {
        let passwords: Vec<String> = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];

        assert_eq!(get_valid(&passwords).len(), 2)
    }

    #[test]
    fn test_is_valid_part_2() {
        let pass1: Password = (1, 3, 'a', "abcde".to_string());
        let pass2: Password = (1, 3, 'b', "cdefg".to_string());

        assert_eq!(is_valid_part_2(&pass1), true);
        assert_eq!(is_valid_part_2(&pass2), false);
    }

    #[test]
    fn test_parse_password() {
        let password: String = "1-3 a: abcde".to_string();
        let parsed = parse_password(&password);
        assert_eq!(parsed, (1, 3, 'a', "abcde".to_string()))
    }

}

pub mod day_1 {
    use super::*;

    pub fn solve(set: HashSet<i64>, sum: i64) -> i64 {
        for num in &set {
            let complement = sum - num;
            
            if set.contains(&complement) {
                return num * complement;
            }
        };

        -1
    }

    pub fn solve_three(set: HashSet<i64>, sum: i64) -> i64 {
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
}

mod utils {
    use std::str::FromStr;

    pub fn read_lines_to_vec<T: FromStr>(filename: &str) -> Vec<T> {
        std::fs::read_to_string(filename)
            .expect("File not found!")
            .trim()
            .lines()
            .map(|x| x.parse().ok().unwrap())  // To Option to avoid needing Debug trait
            .collect()
    }
}


