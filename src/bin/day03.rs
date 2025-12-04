use aoc2025::aoc;

const DAY: u32 = 3;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

fn calc_joltage(line: &[u8], batteries: usize) -> u64 {
    let mut i = 0;
    let mut joltage: u64 = 0;

    for n in 0..batteries {
        let s = if i == 0 { 0 } else { i + 1 };
        for j in s..=line.len() - (batteries - n) {
            if line[j] > *line.get(i).unwrap_or(&b'0') {
                i = j;
            }
        }

        joltage *= 10;
        joltage += (line[i] - b'0') as u64;
        i += 1
    }

    joltage
}

fn part_one(input: &str) -> u64 {
    input.lines()
        .map(|line| line.as_bytes())
        .map(|line| calc_joltage(line, 2))
        .sum()
}

fn part_two(input: &str) -> u64 {
    input.lines()
        .map(|line| line.as_bytes())
        .map(|line| calc_joltage(line, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 357);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 3121910778619);
    }
}
