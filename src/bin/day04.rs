type Input = Vec<Vec<char>>;
type Output = usize;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(input: &Input) -> Output {
    let width = input.len();
    let height = input[0].len();

    let text = "XMAS";

    let mut count = 0;

    // horizontal
    for y in 0..height {
        for x in 0..=width - text.len() {
            if matches!(
                (
                    input[y][x],
                    input[y][x + 1],
                    input[y][x + 2],
                    input[y][x + 3]
                ),
                ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X')
            ) {
                count += 1;
            }
        }
    }

    // vertical
    for y in 0..=height - text.len() {
        for x in 0..width {
            if matches!(
                (
                    input[y][x],
                    input[y + 1][x],
                    input[y + 2][x],
                    input[y + 3][x]
                ),
                ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X')
            ) {
                count += 1;
            }
        }
    }

    // top left - bottom right diagonal
    for y in 0..=height - text.len() {
        for x in 0..=width - text.len() {
            if matches!(
                (
                    input[y][x],
                    input[y + 1][x + 1],
                    input[y + 2][x + 2],
                    input[y + 3][x + 3]
                ),
                ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X')
            ) {
                count += 1;
            }
        }
    }

    // bottom left - top right diagonal
    for y in text.len() - 1..height {
        for x in 0..=width - text.len() {
            if matches!(
                (
                    input[y][x],
                    input[y - 1][x + 1],
                    input[y - 2][x + 2],
                    input[y - 3][x + 3]
                ),
                ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X')
            ) {
                count += 1;
            }
        }
    }

    count
}

fn part_two(input: &Input) -> Output {
    let mut sum = 0;

    let width = input.len();
    let height = input[0].len();

    for y in 1..=height - 3 + 1 {
        for x in 1..=width - 3 + 1 {
            if input[y][x] == 'A' {
                let tl = input[y - 1][x - 1];
                let tr = input[y - 1][x + 1];
                let bl = input[y + 1][x - 1];
                let br = input[y + 1][x + 1];

                if ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
                    && ((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M'))
                {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn main() {
    let input = include_str!("day04_input.txt");
    let parsed_input = parse_input(input);

    println!("Part 1: {}", part_one(&parsed_input));
    println!("Part 2: {}", part_two(&parsed_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn diagonal() {
        let simple_input = r"X...
.M..
..A.
...S";

        assert_eq!(1, part_one(&parse_input(simple_input)));
    }

    #[test]
    fn part_one_example() {
        let parsed_input = parse_input(EXAMPLE_INPUT);
        assert_eq!(18, part_one(&parsed_input));
    }

    #[test]
    fn part_two_example() {
        let x = r"S.S
.A.
M.M";

        let parsed_input = parse_input(x);
        assert_eq!(1, part_two(&parsed_input));
    }
}
