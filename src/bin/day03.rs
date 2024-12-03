use regex::Regex;

type Input = String;
type Output = usize;

fn parse_input(input: &str) -> Input {
    input.to_owned()
}

fn part_one(parsed_input: &Input) -> Output {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(&parsed_input).map(|c| c.extract()) {
        sum += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
    }

    sum
}

fn part_two(parsed_input: &Input) -> Output {
    let re = Regex::new(r"((?:mul)|(?:don't)|(?:do))\((\d*),?(\d*)\)").unwrap();

    let mut toggle = true;
    let mut sum = 0;
    for (_, [a, b, c]) in re.captures_iter(&parsed_input).map(|c| c.extract()) {
        match a {
            "mul" => {
                if toggle {
                    sum += b.parse::<usize>().unwrap() * c.parse::<usize>().unwrap();
                }
            }
            "do" => {
                toggle = true;
            }
            "don't" => {
                toggle = false;
            }
            _ => panic!("something went wrong"),
        }
    }

    sum
}

fn main() {
    let input = include_str!("day03_input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part_one(&parsed_input));
    println!("Part 2: {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed_input = parse_input(input);

        assert_eq!(161, part_one(&parsed_input));
    }

    #[test]
    fn part_two_example() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let parsed_input = parse_input(input);

        assert_eq!(48, part_two(&parsed_input));
    }
}
