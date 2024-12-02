use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .unzip()
}

fn part_one((left_list, right_list): &(Vec<usize>, Vec<usize>)) -> usize {
    let mut left_list = left_list.clone();
    let mut right_list = right_list.clone();

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part_two((left_list, right_list): &(Vec<usize>, Vec<usize>)) -> usize {
    let mut right_counts = HashMap::new();
    for item in right_list {
        *right_counts.entry(item).or_insert(0) += 1;
    }

    left_list
        .iter()
        .map(|left| left * right_counts.get(&left).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = include_str!("day01_input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part_one(&parsed_input));
    println!("Part 2: {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part_one_example() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(11, part_one(&parsed_input));
    }

    #[test]
    fn part_two_example() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(31, part_two(&parsed_input));
    }
}
