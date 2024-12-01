fn main() {
    let input = include_str!("day01_input.txt");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();
        list1.push(left.parse::<u32>().unwrap());
        list2.push(right.parse::<u32>().unwrap());
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let answer = list1
        .iter()
        .zip(&list2)
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u32>();
    println!("Part 1: {}", answer);

    let mut answer2 = 0;
    for item in list1 {
        answer2 += item * list2.iter().filter(|x| **x == item).count() as u32;
    }
    println!("Part 2: {}", answer2);
}
