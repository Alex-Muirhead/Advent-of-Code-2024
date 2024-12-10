use regex::Regex;

pub enum Inst {
    MUL(u32, u32),
    DO,
    DONT,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(u32, u32)> {
    let re =
        Regex::new(r"(mul)\(([0-9]{1,3}),([0-9]{1,3})\)|(do)\(\)()()|(don't)\(\)()()").unwrap();
    let mut output = Vec::new();
    let mut do_mul = true;
    for line in input.lines() {
        for capture in re.captures_iter(line) {
            let (_, [instr, arg_a, arg_b]) = capture.extract();
            match instr {
                "mul" if do_mul => output.push((arg_a.parse().unwrap(), arg_b.parse().unwrap())),
                "do" => do_mul = true,
                "don't" => do_mul = false,
                _ => continue,
            };
        }
    }
    output
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<(u32, u32)>) -> u32 {
    input.into_iter().fold(0, |acc, (a, b)| acc + a * b)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<(u32, u32)>) -> u32 {
    input.into_iter().fold(0, |acc, (a, b)| acc + a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_extract() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let actual = input_generator(input);

        assert_eq!(actual, vec![(2, 4), (5, 5), (11, 8), (8, 5)])
    }

    #[test]
    fn complex_extract() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let actual = input_generator(input);

        assert_eq!(actual, vec![(2, 4), (8, 5)])
    }

    #[test]
    fn simple_part1() {
        let input = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let actual = solve_part1(&input);

        assert_eq!(actual, 161)
    }
}
