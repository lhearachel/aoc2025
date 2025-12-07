use aoc2025::aoc;

const DAY: u32 = 7;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

fn parse(input: &str) -> (usize, usize) {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let start = lines[0].iter().position(|&c| c == b'S').expect("could not find start position");

    let mut splits = 0;
    let mut quantum = vec![0; lines[0].len()];
    quantum[start] = 1;

    // Already processed the first line (start position)
    // Inputs are alternating lines of splitters and non-splitters
    for row in lines.iter().skip(2).step_by(2) {
        for x in 0..row.len() {
            if quantum[x] > 0 && row[x] == b'^' {
                splits += 1;

                quantum[x - 1] += quantum[x];
                quantum[x + 1] += quantum[x];
                quantum[x] = 0;
            }
        }
    }

    (splits, quantum.iter().sum())
}

fn part_one(input: &str) -> usize {
    parse(input).0
}

fn part_two(input: &str) -> usize {
    parse(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 40);
    }
}
