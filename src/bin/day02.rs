fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn is_report_valid(report: &[usize]) -> bool {
    let is_ascending = report.windows(2).all(|window| window[0] > window[1]);
    let is_descending = report.windows(2).all(|window| window[0] < window[1]);
    let valid_diffs = report
        .windows(2)
        .all(|window| (1..=3).contains(&window[0].abs_diff(window[1])));

    (is_ascending || is_descending) && valid_diffs
}

fn part_one(parsed_input: &[Vec<usize>]) -> usize {
    parsed_input
        .iter()
        .filter(|report| is_report_valid(report))
        .count()
}

fn part_two(parsed_input: &[Vec<usize>]) -> usize {
    parsed_input
        .iter()
        .filter(|&report| {
            is_report_valid(report)
                || (0..report.len()).any(|i| {
                    let mut copy = report.clone();
                    copy.remove(i);

                    is_report_valid(&copy)
                })
        })
        .count()
}

fn main() {
    let input = include_str!("day02_input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part_one(&parsed_input));
    println!("Part 2: {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_one_example() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(2, part_one(&parsed_input));
    }

    #[test]
    fn part_two_example() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(4, part_two(&parsed_input));
    }
}
