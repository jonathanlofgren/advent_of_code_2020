use std::fs;
use std::collections::HashSet;


fn count_any_question(text: &str) -> Vec<usize> {
    text
        .split("\n\n")
        .map(|s| s.replace("\n", "").chars().collect::<HashSet<_>>())
        .map(|s| s.len())
        .collect()
}

fn count_all_questions(text: &str) -> Vec<usize> {
    let all_sets: Vec<Vec<HashSet<_>>> = text
        .split("\n\n")
        .map(
            |s| s.split("\n").map(|s| s.chars().collect()).collect()
        )
        .collect();

    all_sets.iter().map(
        |sets| {
            let set0 = &sets[0];

            set0.iter().filter(
                |k| sets.iter().all(|s| s.contains(k))
            ).count()
        }
    ).collect()
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