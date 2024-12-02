type ParsedInput = Vec<Vec<usize>>;

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| line.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_report_valid(report: &Vec<usize>) -> bool {
    let is_ascending = report.windows(2).all(|x| x[0] > x[1]);
    let is_descending = report.windows(2).all(|x| x[0] < x[1]);
    let diffs = report.windows(2).all(|x| {
        let d = x[0].abs_diff(x[1]);
        d >= 1 && d <= 3
    });

    (is_ascending || is_descending) && diffs
}

fn part_one(parsed_input: &ParsedInput) -> usize {
    let mut valid_reports = 0;
    for report in parsed_input {
        if is_report_valid(report) {
            valid_reports += 1;
        }
    }

    valid_reports
}

fn part_two(parsed_input: &ParsedInput) -> usize {
    let mut valid_reports = 0;
    for report in parsed_input {
        if is_report_valid(report) {
            valid_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut copy = report.clone();
                copy.remove(i);

                if is_report_valid(&copy) {
                    valid_reports += 1;
                    break;
                }
            }
        }
    }

    valid_reports
}

fn main() {
    let input = include_str!("day02_input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part_one(&parsed_input));
    println!("Part 2: {}", part_two(&parsed_input));
}
