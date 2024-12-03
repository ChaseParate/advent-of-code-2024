use regex::Regex;

fn part_one(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|captures| {
            let (_, [a, b]) = captures.extract();
            a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let re = Regex::new(r"(mul|do(?:n't)?)\((\d*),?(\d*)\)").unwrap();

    re.captures_iter(input)
        .scan(true, |toggle, captures| {
            let (_, [name, a, b]) = captures.extract();

            Some(match name {
                "mul" => toggle.then(|| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()),
                "do" => {
                    *toggle = true;
                    None
                }
                "don't" => {
                    *toggle = false;
                    None
                }
                _ => panic!("invalid instruction name: {name}"),
            })
        })
        .flatten()
        .sum()
}

fn main() {
    let input = include_str!("day03_input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_example() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, part_one(&input));
    }

    #[test]
    fn part_two_example() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part_two(&input));
    }
}
