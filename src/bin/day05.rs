use std::collections::{HashMap, HashSet};

struct Input {
    rules_map: HashMap<usize, HashSet<usize>>,
    updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();

    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    let mut rules_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (left, right) in rules {
        rules_map.entry(left).or_default().insert(right);
    }

    let updates = lines
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    Input { rules_map, updates }
}

fn is_invalid_update(update: &[usize], rules_map: &HashMap<usize, HashSet<usize>>) -> bool {
    update.iter().enumerate().any(|(i, element)| {
        rules_map.get(element).is_some_and(|next_elements| {
            update[..i]
                .iter()
                .any(|previous_element| next_elements.contains(previous_element))
        })
    })
}

fn part_one(Input { rules_map, updates }: &Input) -> usize {
    updates
        .iter()
        .filter(|update| !is_invalid_update(update, rules_map))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn topological_sort(update: &[usize], rules_map: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    let mut in_degrees: HashMap<usize, usize> = rules_map
        .iter()
        .filter(|(element, _)| update.contains(element))
        .flat_map(|(_, next_elements)| {
            next_elements
                .iter()
                .filter(|next_element| update.contains(next_element))
        })
        .fold(HashMap::new(), |mut map, next_element| {
            *map.entry(*next_element).or_default() += 1;
            map
        });

    let mut sources = update
        .iter()
        .filter(|element| {
            in_degrees
                .get(element)
                .is_none_or(|in_degree| *in_degree == 0)
        })
        .copied()
        .collect::<Vec<_>>();

    let mut sorted = Vec::with_capacity(update.len());

    while let Some(entry) = sources.pop() {
        sorted.push(entry);

        if let Some(edges) = rules_map.get(&entry) {
            for edge in edges {
                if let Some(in_degree) = in_degrees.get_mut(edge) {
                    *in_degree -= 1;

                    if *in_degree == 0 {
                        sources.push(*edge);
                    }
                }
            }
        }
    }

    sorted
}

fn part_two(Input { rules_map, updates }: &Input) -> usize {
    updates
        .iter()
        .filter(|update| is_invalid_update(update, rules_map))
        .map(|update| {
            let fixed_update = topological_sort(update, rules_map);
            fixed_update[fixed_update.len() / 2]
        })
        .sum()
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
