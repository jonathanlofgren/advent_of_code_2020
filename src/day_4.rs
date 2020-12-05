use std::fs;
use regex::Regex;
#[allow(unused_imports)] 
use std::iter::FromIterator;
use std::collections::HashMap;

type Passport = HashMap<String, String>;

fn count_valid_passports(passports: &[Passport], strict: bool) -> usize {
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

fn get_passports(lines: &[&str]) -> Vec<Passport> {
    let regex = Regex::new(r"([a-z]{3}:\S+)").unwrap();

    let key_sets: Vec<Passport> = lines.iter().map(|l| {
        let captures = regex.captures_iter(l).collect::<Vec<_>>();
        
        captures
            .iter()
            .map(|c| {
                let key_value = c.get(1).unwrap().as_str().to_string();
                let split: Vec<_> = key_value.split(':').collect();

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