pub struct Puzzle {
    pub rows: Vec<Vec<char>>,
    pub size: (usize, usize),
}

impl Puzzle {
    fn new(rows: Vec<Vec<char>>) -> Self {
        // Ensure the initial puzzle is the right shape
        let mut row_lengths: Vec<usize> = rows.iter().map(|row| row.len()).collect();
        let expected = row_lengths.pop().expect("Puzzle must have at least 1 row!");
        if row_lengths.iter().any(|&v| v != expected) {
            panic!("This isn't right!")
        }
        let size = (row_lengths.len() + 1, expected);
        Puzzle { rows, size }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Puzzle {
    let rows = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    Puzzle::new(rows)
}

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[aoc(day4, part1)]
pub fn solve_part1(input: &Puzzle) -> u32 {
    let (rows, cols) = input.size;
    let word_length = 4;
    let mut counts = 0;

    for y in 0..rows {
        for x in 0..cols {
            'dir: for (dx, dy) in DIRECTIONS {
                let mut word = Vec::new();
                let (mut nx, mut ny) = (x as isize, y as isize);
                for _ in 0..word_length {
                    if nx < 0 || nx >= (cols as isize) || ny < 0 || ny >= (rows as isize) {
                        continue 'dir;
                    }
                    word.push(input.rows[ny as usize][nx as usize]);
                    nx += dx;
                    ny += dy;
                }
                if word == vec!['X', 'M', 'A', 'S'] {
                    counts += 1;
                }
            }
        }
    }

    counts
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Puzzle) -> u32 {
    let (rows, cols) = input.size;
    let mut counts = 0;

    for y in 0..rows {
        for x in 0..cols {
            // Check for a center on 'A
            if input.rows[y][x] != 'A' {
                continue;
            }
            // And that we have a box around us to search
            if x < 1 || x >= cols - 1 || y < 1 || y >= rows - 1 {
                continue;
            }
            let diag_one = [
                input.rows[y + 1][x + 1],
                input.rows[y][x],
                input.rows[y - 1][x - 1],
            ];
            let diag_two = [
                input.rows[y + 1][x - 1],
                input.rows[y][x],
                input.rows[y - 1][x + 1],
            ];
            if (diag_one == ['M', 'A', 'S'] || diag_one == ['S', 'A', 'M'])
                && (diag_two == ['M', 'A', 'S'] || diag_two == ['S', 'A', 'M'])
            {
                counts += 1
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_puzzle() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let puzzle = input_generator(input);
        println!("Puzzle size: {:?}", puzzle.size);
        let actual = solve_part1(&puzzle);
        assert_eq!(actual, 4)
    }

    #[test]
    fn bigger_puzzle() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let puzzle = input_generator(input);
        println!("Puzzle size: {:?}", puzzle.size);
        let actual = solve_part1(&puzzle);
        assert_eq!(actual, 18)
    }

    #[test]
    fn bigger_crosses() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        let puzzle = input_generator(input);
        println!("Puzzle size: {:?}", puzzle.size);
        let actual = solve_part2(&puzzle);
        assert_eq!(actual, 9)
    }
}
