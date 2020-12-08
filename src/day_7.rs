use crate::utils;
use regex::Regex;
use std::str::FromStr;
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
struct Bags(usize, String);

#[derive(Debug, PartialEq)]
struct Rule {
    bag: String,
    can_contain: Option<Vec<Bags>>
}


impl FromStr for Bags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^(\d+) (.+?) bag[s]*[.]*$").unwrap();
        let caps = regex.captures(s).unwrap();
        let n = caps.get(1).unwrap().as_str().parse().unwrap();
        let bag = caps.get(2).unwrap().as_str().to_string();

        Ok(Bags(n, bag))
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split(" bags contain ").collect();
        let can_contain: Option<Vec<Bags>> = match split[1] {
            "no other bags." => None,
            s => Some(s.split(", ").map(|s| s.parse().unwrap()).collect())
        };

        Ok(Self {
            bag: split[0].to_string(),
            can_contain: can_contain
        })
    }
}


fn get_rule_map(rules: &Vec<Rule>) -> HashMap<String, &Vec<Bags>> {
    rules
        .iter()
        .filter(|r| r.can_contain.is_some())
        .map(|r| (r.bag.clone(), r.can_contain.as_ref().unwrap().clone()))
        .collect::<HashMap<_, _>>()
}


fn number_of_bags_containing(rule_map: &HashMap<String, &Vec<Bags>>, bag: &str) -> usize {
    rule_map
        .keys()
        .filter(|b| *b != bag && can_contain(b, bag, &rule_map))
        .count()
}


fn can_contain(base_bag: &str, contain: &str, rule_map: &HashMap<String, &Vec<Bags>>) -> bool {
    match (base_bag == contain, rule_map.get(base_bag)) {
        (true, _) => true,
        (false, None) => false,
        (false, Some(bags)) => bags.iter().any(|b| can_contain(&b.1, contain, rule_map)),
    }
}


fn number_of_bags_in(rule_map: &HashMap<String, &Vec<Bags>>, bag: &str) -> usize {
    match rule_map.get(bag) {
        Some(bags) => bags.iter().map(|b| b.0 + b.0 * number_of_bags_in(rule_map, &b.1)).sum(),
        None => 0,
    }
}


pub fn main() {
    let rules: Vec<Rule> = utils::read_lines_to_vec("data/day_7.txt");
    let rule_map = get_rule_map(&rules);

    println!("======== Day 7 ========");
    println!("Part 1 = {}", number_of_bags_containing(&rule_map, "shiny gold"));
    println!("Part 2 = {}", number_of_bags_in(&rule_map, "shiny gold"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bags() {
        let bags1: Bags = "1 dim aqua bag".parse().unwrap();
        let bags2: Bags = "5 posh fuchsia bags".parse().unwrap();
        let bags3: Bags = "2 blue bagish bags.".parse().unwrap();

        assert_eq!(bags1, Bags(1, "dim aqua".to_string()));
        assert_eq!(bags2, Bags(5, "posh fuchsia".to_string()));
        assert_eq!(bags3, Bags(2, "blue bagish".to_string()));
    }

    #[test]
    fn test_rule() {
        let rule1: Rule = "mirrored maroon bags contain 3 drab coral bags.".parse().unwrap();
        let rule2: Rule = "muted bronze bags contain 3 dull aqua bags, 4 striped turquoise bags.".parse().unwrap();
        let rule3: Rule = "greenish bags contain no other bags.".parse().unwrap();

        assert_eq!(
            rule1,
            Rule {
                bag: "mirrored maroon".to_string(),
                can_contain: Some(vec![Bags(3, "drab coral".to_string())])}
        );

        assert_eq!(
            rule2,
            Rule {
                bag: "muted bronze".to_string(),
                can_contain: Some(vec![Bags(3, "dull aqua".to_string()), Bags(4, "striped turquoise".to_string())])
            }
        );

        assert_eq!(
            rule3,
            Rule {
                bag: "greenish".to_string(),
                can_contain: None
            }
        );
    }

    #[test]
    fn test_number_of_bags_containing() {
        let rules: Vec<Rule> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".parse().unwrap(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".parse().unwrap(),
            "bright white bags contain 1 shiny gold bag.".parse().unwrap(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".parse().unwrap(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".parse().unwrap(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".parse().unwrap(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".parse().unwrap(),
            "faded blue bags contain no other bags.".parse().unwrap(),
            "dotted black bags contain no other bags.".parse().unwrap(),
        ];
        let rule_map = get_rule_map(&rules);

        let num = number_of_bags_containing(&rule_map, "shiny gold");

        assert_eq!(num, 4);
    }

    #[test]
    fn test_number_of_bags_in() {
        let rules: Vec<Rule> = vec![
            "shiny gold bags contain 2 dark red bags.".parse().unwrap(),
            "dark red bags contain 2 dark orange bags.".parse().unwrap(),
            "dark orange bags contain 2 dark yellow bags.".parse().unwrap(),
            "dark yellow bags contain 2 dark green bags.".parse().unwrap(),
            "dark green bags contain 2 dark blue bags.".parse().unwrap(),
            "dark blue bags contain 2 dark violet bags.".parse().unwrap(),
            "dark violet bags contain no other bags.".parse().unwrap(),
        ];
        let rule_map = get_rule_map(&rules);

        let num = number_of_bags_in(&rule_map, "shiny gold");

        assert_eq!(num, 126);
    }
}