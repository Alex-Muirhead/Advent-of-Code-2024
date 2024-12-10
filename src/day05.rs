use std::collections::HashMap;

pub struct PrintingSpec {
    ordering: HashMap<u32, Vec<u32>>,
    prints: Vec<Vec<u32>>,
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> PrintingSpec {
    let mut lines = input.lines();

    let spec_lines: Vec<&str> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
    let print_lines: Vec<&str> = lines.collect();

    let mut ordering = HashMap::<u32, Vec<u32>>::new();
    spec_lines
        .iter()
        .map(|spec| spec.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .for_each(|(before, after)| {
            ordering
                .entry(after)
                .and_modify(|values| values.push(before))
                .or_insert(vec![before]);
        });

    let prints = print_lines
        .iter()
        .map(|line| line.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    PrintingSpec { ordering, prints }
}

#[aoc(day5, part1)]
fn solve_part1(input: &PrintingSpec) -> u32 {
    let mut total = 0;
    'print: for print in input.prints.clone() {
        for (order, page) in print.iter().enumerate() {
            let Some(check_list) = input.ordering.get(&page) else {
                // If the page doesn't have any rules, move to the next page
                continue;
            };
            let trailing_pages = &print[order..];
            for check_page in check_list {
                if trailing_pages.contains(check_page) {
                    continue 'print;
                }
            }
        }
        let print_length = print.len();
        total += print[print_length.div_ceil(2) - 1];
    }

    total
}

#[aoc(day5, part2)]
fn solve_part2(input: &PrintingSpec) -> u32 {
    let mut total = 0;
    for mut print in input.prints.clone() {
        let mut contains_error = false;
        let mut position = 0;
        'page: while position < print.len() {
            let page = print[position];
            let Some(check_list) = input.ordering.get(&page) else {
                // If the page doesn't have any rules, move to the next page
                position += 1;
                continue 'page;
            };
            let trailing_pages = &mut print[position..];
            // Swap the *first* one
            let bad_page = trailing_pages.iter().position(|tp| check_list.contains(tp));
            if let Some(page_position) = bad_page {
                trailing_pages.swap(0, page_position);
                contains_error = true;
            } else {
                position += 1;
            }
        }
        if contains_error {
            let print_length = print.len();
            total += print[print_length.div_ceil(2) - 1];
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_spec_parse() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let spec = input_generator(&input);

        // Check one of the rule-sets
        assert_eq!(*spec.ordering.get(&47).unwrap(), vec![97, 75]);

        // Check the print lines
        assert_eq!(spec.prints[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(spec.prints[2], vec![75, 29, 13]);

        let actual = solve_part1(&spec);
        assert_eq!(actual, 143);

        let actual = solve_part2(&spec);
        assert_eq!(actual, 123);
    }
}
