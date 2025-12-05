use aoc2025::aoc;

const DAY: u32 = 5;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (lines_fresh, lines_ingredients) = input.split_once("\n\n").unwrap();

    let mut fresh = lines_fresh.lines()
        .map(|line| {
            let (head, tail) = line.split_once('-').unwrap();
            (head.parse().unwrap(), tail.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();
    fresh.sort();

    let mut merged = Vec::from([fresh[0]]);
    for &(h1, t1) in &fresh[1..] {
        let &(h2, t2) = merged.last().unwrap();
        if h1 > t2 {
            merged.push((h1, t1));
        } else {
            *merged.last_mut().unwrap() = (h2, t2.max(t1));
        }
    }

    let ingredients = lines_ingredients.lines()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<usize>>();

    (merged, ingredients)
}

fn part_one(input: &str) -> usize {
    let (fresh, ingredients) = parse(input);

    ingredients.iter()
        .filter(|i| fresh.iter().any(|(h, t)| h <= i && t >= i))
        .count()
}

fn part_two(input: &str) -> usize {
    let (fresh, _) = parse(input);
    fresh.iter().map(|(h, t)| t - h + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 14);
    }
}
