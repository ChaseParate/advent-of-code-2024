use std::collections::{HashMap, HashSet};

type Input = (Vec<(usize, usize)>, Vec<Vec<usize>>);
type Output = usize;

fn parse_input(input: &str) -> Input {
    let (left, right) = input
        .split_once("\r\n\r\n")
        .or(input.split_once("\n\n"))
        .unwrap();

    let rules = left
        .lines()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    let printed = right
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, printed)
}

fn part_one((rules, printed): &Input) -> Output {
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (a, b) in rules {
        rules_map.entry(*a).or_default().push(*b);
    }

    let mut sum = 0;

    'main: for seq in printed.iter() {
        for (i, ele) in seq.iter().enumerate() {
            if let Some(entry) = rules_map.get(ele) {
                for prev_ele in &seq[..i] {
                    if entry.contains(prev_ele) {
                        continue 'main;
                    }
                }
            }
        }

        sum += seq[seq.len() / 2];
    }

    sum
}

fn is_valid(seq: &[usize], rules_map: &HashMap<usize, HashSet<usize>>) -> Option<usize> {
    for (i, ele) in seq.iter().enumerate() {
        if let Some(entry) = rules_map.get(ele) {
            for prev_ele in &seq[..i] {
                if entry.contains(prev_ele) {
                    return Some(i);
                }
            }
        }
    }

    None
}

fn topological_sort(
    seq: &[usize],
    rules: &Vec<(usize, usize)>,
    rules_map: &HashMap<usize, HashSet<usize>>,
) -> Vec<usize> {
    let mut indegrees: HashMap<usize, usize> = HashMap::new();
    for (before, after) in rules {
        if seq.contains(before) && seq.contains(after) {
            *indegrees.entry(*after).or_default() += 1;
        }
    }

    let mut zero_indegs = Vec::new();
    for item in seq {
        if *indegrees.get(item).unwrap_or(&0) == 0 {
            zero_indegs.push(*item);
        }
    }

    let mut output = vec![];
    while let Some(entry) = zero_indegs.pop() {
        output.push(entry);
        if let Some(edges) = rules_map.get(&entry) {
            for edge in edges {
                if let Some(indeg) = indegrees.get_mut(edge) {
                    *indeg -= 1;

                    if *indeg == 0 {
                        zero_indegs.push(*edge);
                    }
                }
            }
        }
    }

    output
}

fn part_two((rules, printed): &Input) -> Output {
    let mut rules_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (a, b) in rules {
        rules_map.entry(*a).or_default().insert(*b);
    }

    let mut sum = 0;

    for seq in printed {
        if is_valid(seq, &rules_map).is_some() {
            let new = topological_sort(seq, rules, &rules_map);
            dbg!(&new);
            sum += new[new.len() / 2];
        }
    }

    sum
}

fn main() {
    let input = include_str!("day05_input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part_one(&parsed_input));
    println!("Part 2: {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_one_example() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(143, part_one(&parsed_input));
    }

    #[test]
    fn part_two_example() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(123, part_two(&parsed_input));
    }

    #[test]
    fn my_example() {
        let input = r"2|1
1|3
1|4
4|3
5|6
6|5

1,2,3,4";
        let parsed_input = parse_input(input);
        assert_eq!(4, part_two(&parsed_input));
    }
}
