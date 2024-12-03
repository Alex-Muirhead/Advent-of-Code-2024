use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let pairs = input
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_ascii_whitespace().collect();
            let first = *words.first().unwrap();
            let second = *words.last().unwrap();
            (
                first.parse::<u32>().unwrap(),
                second.parse::<u32>().unwrap(),
            )
        })
        .collect();
    pairs
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list_one = input.0.clone();
    let mut list_two = input.1.clone();

    list_one.sort();
    list_two.sort();

    list_one
        .into_iter()
        .zip(list_two)
        .fold(0, |value, (a, b)| value + (a as i32).abs_diff(b as i32))
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list_one = input.0.clone();
    let mut list_two = input.1.clone();

    list_one.sort();
    list_two.sort();

    let mut counter = HashMap::new();

    list_two.into_iter().for_each(|val| {
        counter
            .entry(val)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });

    let mut total = 0;
    list_one.into_iter().for_each(|val| {
        total += val * (*counter.entry(val).or_default());
    });

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input() {
        let input = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";

        let output = input_generator(input);
        assert_eq!(output.0, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(output.1, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn part1_calc() {
        let input_one = vec![3, 4, 2, 1, 3, 3];
        let input_two = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(solve_part1(&(input_one, input_two)), 11)
    }

    #[test]
    fn part2_calc() {
        let input_one = vec![3, 4, 2, 1, 3, 3];
        let input_two = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(solve_part2(&(input_one, input_two)), 31)
    }
}
