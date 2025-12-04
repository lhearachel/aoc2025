use std::cmp::{max, min};
use std::collections::HashSet;

use aoc2025::aoc;

const DAY: u32 = 2;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(s, e)| (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap()))
        .collect()
}

fn len_digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn multiplier(seed: u32, repeats: u32) -> u64 {
    (0..repeats).fold(0, |acc, i| acc + e10(i * seed))
}

fn e10(e: u32) -> u64 {
    10u64.pow(e)
}

// "Repeating" numbers are those of the form `R = N || N`. Given that `N` has `D` digits:
//   R = (N * 10^D) + N
//     = N * (10^D + 1)
//
// So, for a given `D`, valid values of `N` within the range `[H, T]` must fall within:
//   - `max(10^(D-1), ceil(H / (10^D + 1)))`
//   - `min(10^D - 1, floor(T / (10^D + 1)))`
//
// Then we just use the arithmetic progression to compute the sum and multiply by `10^D + 1`.
fn sum_repeats(head: u64, tail: u64) -> u64 {
    let min_d = len_digits(head).div_ceil(2);
    let max_d = len_digits(tail) / 2;

    (min_d..=max_d).map(|d| {
        let m = multiplier(d, 2);
        let h = max(e10(d - 1), head.div_ceil(m));
        let t = min(e10(d) - 1, tail / m);

        if h > t {
            return 0
        }

        let n_terms = t - h + 1;
        let s_terms = n_terms * (h + t) / 2;
        s_terms * m
    }).sum()
}

fn part_one(input: &str) -> u64 {
    parse(input).iter().map(|(h, t)| sum_repeats(*h, *t)).sum()
}

fn part_two(input: &str) -> u64 {
    let mut seen = HashSet::new();

    parse(input).iter().flat_map(|(h, t)| {
        let max_d = ((*t as f64).log10() + 1.).floor() as u64;
        let max_p = max_d / 2;

        (1..=max_p).flat_map(move |p| {
            let max_r = max_d / p; // possible repeats in the period
            (2..=max_r).flat_map(move |num_p| {
                let pat = (1..num_p).fold(1u64, |acc, _| acc * e10(p as u32) + 1);
                let min_d = h.div_ceil(pat).max(e10(p as u32 - 1));
                let max_d = (t / pat).min(e10(p as u32) - 1);
                (min_d..=max_d).map(move |d| d * pat)
            })
        })
    }).filter(|i| seen.insert(*i)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 1227775554);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 4174379265);
    }
}
