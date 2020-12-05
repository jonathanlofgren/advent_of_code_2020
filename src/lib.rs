use std::collections::HashSet;
use std::collections::HashMap;

pub mod day_4 {
    use super::*;
    use std::fs;
    use regex::Regex;
    use std::iter::FromIterator;

    type Passport = HashMap<String, String>;

    fn count_valid_passports(passports: &Vec<Passport>, strict: bool) -> usize {
        passports
            .iter()
            .map(|p| is_valid(p, strict))
            .filter(|v| v.unwrap_or(false))
            .count()
    }

    fn is_valid(passport: &Passport, strict: bool) -> Option<bool> {
        let byr: i64 = passport.get("byr")?.parse().ok()?;
        let iyr: i64 = passport.get("iyr")?.parse().ok()?;
        let eyr: i64 = passport.get("eyr")?.parse().ok()?;
        let hgt = passport.get("hgt")?;
        let hcl = passport.get("hcl")?;
        let ecl = passport.get("ecl")?;
        let pid = passport.get("pid")?;

        if !strict {
            return Some(true)
        }

        // Check hgt
        let hgt_regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        let caps = hgt_regex.captures(hgt)?;
        let hgt_num: i64 = caps.get(1)?.as_str().parse().ok()?;
        let hgt_unit = caps.get(2)?.as_str();
        let hgt_valid = if hgt_unit == "cm" {
            (hgt_num >= 150) && (hgt_num <= 193)
        } else {
            (hgt_num >= 59) && (hgt_num <= 76)
        };

        let hcl_regex = Regex::new(r"^#[0-9|a-f]{6}$").unwrap();
        let ecl_regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();

        Some(
            (byr >= 1920) && (byr <= 2002) &&
            (iyr >= 2010) && (iyr <= 2020) &&
            (eyr >= 2020) && (eyr <= 2030) &&
            hgt_valid &&
            hcl_regex.is_match(hcl) &&
            ecl_regex.is_match(ecl) &&
            pid_regex.is_match(pid)
        )
    }

    fn get_passports(lines: &Vec<&str>) -> Vec<Passport> {
        let regex = Regex::new(r"([a-z]{3}:\S+)").unwrap();

        let key_sets: Vec<Passport> = lines.iter().map(|l| {
            let captures = regex.captures_iter(l).collect::<Vec<_>>();
            
            captures
                .iter()
                .map(|c| {
                    let key_value = c.get(1).unwrap().as_str().to_string();
                    let split: Vec<_> = key_value.split(":").collect();

                    (split[0].to_string(), split[1].to_string())
                }).collect()
        }).collect();

        key_sets
    }

    pub fn main() {
        let text = fs::read_to_string("data/day_4.txt").expect("File not found");
        let lines: Vec<&str> = text.split("\n\n").collect();
        let passports = get_passports(&lines);

        println!("======== Day 4 ========");
        println!("Part 1 = {}", count_valid_passports(&passports, false));
        println!("Part 2 = {}", count_valid_passports(&passports, true));
    }

    #[test]
    fn test_get_key_sets() {
        let lines = vec![
            "aaa:1 bbb:a\nqqq:4",
            "ccc:1\nqqq:4",
        ];

        let expected: Vec<Passport> = vec![
            HashMap::from_iter(vec![
                ("aaa".to_string(), "1".to_string()),
                ("bbb".to_string(), "a".to_string()),
                ("qqq".to_string(), "4".to_string()),
            ]),
            HashMap::from_iter(vec![
                ("ccc".to_string(), "1".to_string()),
                ("qqq".to_string(), "4".to_string()),
            ]),
        ];
        let passorts = get_passports(&lines);

        assert_eq!(passorts, expected);
    }
}

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


