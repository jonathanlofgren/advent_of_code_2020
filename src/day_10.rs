use crate::utils;


fn get_jolt_differences(jolts: &[usize]) -> Vec<usize> {
    let mut counts = vec![0; 4];

    for pair in jolts.windows(2) {
        counts[pair[1] - pair[0]] += 1
    }

    counts
}


fn number_of_paths(jolts: &[usize]) -> usize {
    let mut m: Vec<usize> = vec![0; jolts.len()];

    // base case
    m[jolts.len() - 1] = 1;

    for (i, val) in jolts.iter().enumerate().rev().skip(1) {
        m[i] = 
            jolts.get(i+1).filter(|&v| *v <= val + 3).map_or(0, |_| m[i+1]) +
            jolts.get(i+2).filter(|&v| *v <= val + 3).map_or(0, |_| m[i+2]) +
            jolts.get(i+3).filter(|&v| *v <= val + 3).map_or(0, |_| m[i+3]);
    }

    m[0]
}


pub fn main() {
    let mut jolts: Vec<usize> = utils::read_lines_to_vec("data/day_10.txt");
    jolts.sort();
    jolts.insert(0, 0);
    jolts.push(*jolts.last().unwrap() + 3);

    let diffs = get_jolt_differences(&jolts);

    println!("======== Day 10 ========");
    println!("Part 1 = {}", diffs[1] * diffs[3]);
    println!("Part 2 = {}", number_of_paths(&jolts));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_jolt_differences() {
        let jolts: Vec<usize> = vec![0, 1, 2, 5, 7, 10, 11, 14];
        let diffs = get_jolt_differences(&jolts);

        assert_eq!(diffs, vec![0, 3, 1, 3]);

    }

    #[test]
    fn test_number_of_paths() {
        let mut jolts: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        jolts.sort();

        assert_eq!(number_of_paths(&jolts), 8);
    }

}