use crate::utils;
use std::collections::HashSet;


fn first_non_allowed(numbers: &[i64], preamble: i64) -> Option<i64> {
    let mut set: HashSet<i64> = numbers[..preamble as usize].iter().cloned().collect();

    for current in preamble as usize..numbers.len() {
        if !has_pair_sum(&set, numbers[current]) {
            return Some(numbers[current])
        }
        set.remove(&numbers[current-preamble as usize]);
        set.insert(numbers[current]);
    }

    None
}


fn has_pair_sum(elements: &HashSet<i64>, sum: i64) -> bool {
    for e in elements.iter() {
        let comp = sum - e;
        if comp != *e && elements.contains(&comp) {
            return true
        }
    }

    false
}


fn find_consecutive_sum(numbers: &[i64], total: i64) -> Option<(usize, usize)> {

    for start in 0..numbers.len()-1 {
        let mut sum = numbers[start];

        for i in start+1..numbers.len() {
            sum += numbers[i];

            if sum == total {
                return Some((start, i+1));
            } else if sum > total {
                break;
            }

        }
    }

    None
}



pub fn main() {
    let numbers: Vec<i64> = utils::read_lines_to_vec("data/day_9.txt");
    let part_1 = first_non_allowed(&numbers, 25).unwrap();
    let (start, end) = find_consecutive_sum(&numbers, part_1).unwrap();
    let part_2 = numbers[start..end].iter().min().unwrap() + numbers[start..end].iter().max().unwrap();

    println!("======== Day 9 ========");
    println!("Part 1 = {}", part_1);
    println!("Part 2 = {:?}", part_2);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let numbers: Vec<i64> = vec![
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576,
        ];

        let first = first_non_allowed(&numbers, 5);

        assert_eq!(first, Some(127));
    }

}