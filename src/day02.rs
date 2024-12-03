type Report = Vec<u32>;

pub struct Data {
    pub reports: Vec<Report>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Data {
    let reports = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    Data { reports }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Data) -> u32 {
    input.reports.iter().filter(|r| check_report(r)).count() as u32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Data) -> u32 {
    input
        .reports
        .iter()
        .filter(|r| brute_check_report(r))
        .count() as u32
}

pub fn check_report(report: &Report) -> bool {
    let deltas: Vec<i32> = report
        .windows(2)
        .map(|s| s[0] as i32 - s[1] as i32)
        .collect();
    let start = deltas.first().unwrap().signum();
    deltas
        .iter()
        .all(|d| d.signum() == start && d.abs() >= 1 && d.abs() <= 3)
}

pub fn brute_check_report(report: &Report) -> bool {
    let n = report.len();
    for i in 0..n {
        let mut test_report = report.clone();
        test_report.remove(i);
        let check = check_report(&test_report);
        if check {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_report_1() {
        let input = vec![7, 6, 4, 2, 1];
        // Deltas: -1, -2, -2, -1
        assert!(check_report(&input));
    }

    #[test]
    fn check_report_2() {
        let input = vec![1, 2, 7, 8, 9];
        // Deltas: +1, +5, +1, +2
        assert!(!check_report(&input));
    }

    #[test]
    fn check_report_3() {
        let input = vec![9, 7, 6, 2, 1];
        // Deltas:         -2, -1, -4, -1
        assert!(!check_report(&input));
    }

    #[test]
    fn check_report_4() {
        let input = vec![1, 3, 2, 4, 5];
        // Deltas: +2, -1, +2, +3
        assert!(!check_report(&input));
    }

    #[test]
    fn check_report_5() {
        let input = vec![8, 6, 4, 4, 1];
        // Deltas: -2, -2, 0, -3
        assert!(!check_report(&input));
    }

    #[test]
    fn check_report_6() {
        let input = vec![1, 3, 6, 7, 9];
        assert!(check_report(&input));
    }
}
