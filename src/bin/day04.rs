fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_matches(
    input: &[Vec<char>],
    text: &[char],
    extract: impl Fn(usize, usize) -> Option<Vec<char>>,
) -> usize {
    let width = input.len();
    let height = input[0].len();

    let reversed_text = text.iter().copied().rev().collect::<Vec<_>>();

    (0..width)
        .flat_map(|y| (0..height).map(move |x| (x, y)))
        .filter(|(x, y)| {
            extract(*x, *y).is_some_and(|sequence| sequence == text || sequence == reversed_text)
        })
        .count()
}

fn part_one(input: &[Vec<char>]) -> usize {
    let width = input.len();
    let height = input[0].len();

    let text = "XMAS".chars().collect::<Vec<_>>();
    let len = text.len();

    let horizontal = |x: usize, y: usize| {
        (x <= width - len).then(|| (0..len).map(|i| input[y][x + i]).collect::<Vec<_>>())
    };
    let vertical = |x: usize, y: usize| {
        (y <= height - len).then(|| (0..len).map(|i| input[y + i][x]).collect::<Vec<_>>())
    };
    let diagonal_tl_br = |x: usize, y: usize| {
        (x <= width - len && y <= height - len)
            .then(|| (0..len).map(|i| input[y + i][x + i]).collect::<Vec<_>>())
    };
    let diagonal_bl_tr = |x: usize, y: usize| {
        (x <= width - len && y + 1 >= len && y < height)
            .then(|| (0..len).map(|i| input[y - i][x + i]).collect::<Vec<_>>())
    };

    count_matches(input, &text, horizontal)
        + count_matches(input, &text, vertical)
        + count_matches(input, &text, diagonal_tl_br)
        + count_matches(input, &text, diagonal_bl_tr)
}

fn part_two(input: &[Vec<char>]) -> usize {
    let mut sum = 0;

    let width = input.len();
    let height = input[0].len();

    for y in 1..=height - 2 {
        for x in 1..=width - 2 {
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
