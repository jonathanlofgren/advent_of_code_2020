use std::str::FromStr;

pub fn read_lines_to_vec<T: FromStr>(filename: &str) -> Vec<T> {
    std::fs::read_to_string(filename)
        .expect("File not found!")
        .trim()
        .lines()
        .map(|x| x.parse().ok().unwrap())  // To Option to avoid needing Debug trait
        .collect()
}
