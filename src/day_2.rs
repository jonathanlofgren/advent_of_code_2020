use regex::Regex;
use crate::utils;

type Password = (usize, usize, char, String);

fn get_valid(passwords: &[String]) -> Vec<Password> {
    passwords.iter().map(|x| parse_password(x)).filter(|x| is_valid(x)).collect()
}

fn is_valid(password: &Password) -> bool {
    let matches = password.3.matches(password.2).count();
    password.0 <= matches && matches <= password.1
}

fn get_valid_part_2(passwords: &[String]) -> Vec<Password> {
    passwords.iter().map(|x| parse_password(x)).filter(|x| is_valid_part_2(x)).collect()
}

fn is_valid_part_2(password: &Password) -> bool {
    let chars: Vec<char> = password.3.chars().collect();
    (chars[password.0 - 1] == password.2) ^ (chars[password.1 - 1] == password.2)
}

fn parse_password(password: &str) -> Password {
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