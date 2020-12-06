use std::fs;
use std::collections::HashSet;


fn count_any_question(text: &str) -> Vec<usize> {
    text
        .split("\n\n")
        .map(|s| s.replace("\n", "").chars().collect::<HashSet<_>>().len())
        .collect()
}

fn count_all_questions(text: &str) -> Vec<usize> {
    text
        .split("\n\n")
        .map(
            |x| {
                let sets: Vec<HashSet<_>> = x.split("\n").map(|s| s.chars().collect()).collect();
                let set0 = sets[0].clone();

                sets[1..]
                    .iter()
                    .fold(set0, |l, r| l.intersection(r).cloned().collect::<HashSet<_>>())
                    .len()
            }
        )
        .collect()
}

pub fn main() {
    let text = fs::read_to_string("data/day_6.txt").expect("File not found").trim().to_string();
    let num_any_question: usize = count_any_question(&text).iter().sum();
    let num_all_questions: usize = count_all_questions(&text).iter().sum();

    println!("======== Day 6 ========");
    println!("Part 1 = {}", num_any_question);
    println!("Part 2 = {}", num_all_questions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_any_question() {
        let text = "ab\nbc\nq\n\nb\nd\nq\n";
        assert_eq!(count_any_question(text), vec![4, 3]);
    }

    #[test]
    fn test_count_all_questions() {
        let text = "ab\nbc\nbqa\n\nb\ndb\nq\n";
        assert_eq!(count_all_questions(text), vec![1, 0]);
    }
}