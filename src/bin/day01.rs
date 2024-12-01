fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut lists = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        lists.push((left.parse().unwrap(), right.parse().unwrap()));
    }

    lists
}

fn part_one(parsed_input: &[(usize, usize)]) -> usize {
    let (mut left_list, mut right_list): (Vec<_>, Vec<_>) = parsed_input.iter().copied().unzip();

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn part_two(parsed_input: &[(usize, usize)]) -> usize {
    let (left_list, right_list): (Vec<_>, Vec<_>) = parsed_input.iter().copied().unzip();

    left_list
        .into_iter()
        .map(|left| left * right_list.iter().filter(|right| left == **right).count())
        .sum()
}

fn main() {
    let input = include_str!("day01_input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part_one(&parsed_input));
    println!("Part 2: {}", part_two(&parsed_input));
}
