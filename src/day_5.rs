use crate::utils;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Seat {
    row: usize,
    column: usize
}

impl FromStr for Seat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_binary = s[0..7].replace("F", "0").replace("B", "1");
        let column_binary = s[7..].replace("L", "0").replace("R", "1");

        Ok(Self {
            row: usize::from_str_radix(&row_binary, 2)?,
            column: usize::from_str_radix(&column_binary, 2)?
        })
    }
}

impl Seat {
    fn id(&self) -> usize {
        8 * self.row + self.column
    }
}

fn find_first_missing_seat(seats: &[usize]) -> Option<usize> {
    let mut seats = seats.to_vec();
    seats.sort();
    
    for pair in seats.windows(2) {
        if pair[1] != pair[0] + 1 {
            return Some(pair[0] + 1)
        }
    }

    None
}


pub fn main() {
    let seats: Vec<Seat> = utils::read_lines_to_vec("data/day_5.txt");
    let seat_ids: Vec<usize> = seats.iter().map(|s| s.id()).collect();

    println!("======== Day 5 ========");
    println!("Part 1 = {}", seat_ids.iter().max().unwrap());
    println!("Part 2 = {}", find_first_missing_seat(&seat_ids).unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_from_string_and_id() {
        let seat1: Seat = "BFFFBBFRRR".parse().unwrap();
        let seat2: Seat = "FFFBBBFRRR".parse().unwrap();
        let seat3: Seat = "BBFFBBFRLL".parse().unwrap();

        assert_eq!(seat1, Seat{row: 70, column: 7});
        assert_eq!(seat2, Seat{row: 14, column: 7});
        assert_eq!(seat3, Seat{row: 102, column: 4});

        assert_eq!(seat1.id(), 8 * 70 + 7);
    }

    #[test]
    fn test_find_missing_seat() {
        let seats_missing = vec![4, 5, 6, 8, 9];
        let seats_full = vec![100, 101, 102, 103, 104];
        let seats_empty = vec![];
        let seats_many_missing = vec![0, 1, 2, 3, 100];

        assert_eq!(find_first_missing_seat(&seats_missing), Some(7));
        assert_eq!(find_first_missing_seat(&seats_full), None);
        assert_eq!(find_first_missing_seat(&seats_empty), None);
        assert_eq!(find_first_missing_seat(&seats_many_missing), Some(4));
    }
}
