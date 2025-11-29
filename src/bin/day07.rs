use aoc2025::aoc;

const DAY: u32 = 7;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

fn parse(input: &str) -> () {
    todo!()
}

fn part_one(input: &str) -> u32 {
    parse(input);
    0
}

fn part_two(input: &str) -> u32 {
    parse(input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 0);
    }
}
