use aoc2025::aoc;

const DAY: u32 = 6;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

enum Op {
    Add,
    Mul,
}

enum ParseModel {
    Human,
    Cephalopod,
}

fn parse_human(starts: &[usize], bytes: &[&[u8]]) -> (Vec<usize>, Vec<u64>) {
    let mut terms: Vec<u64> = vec![0; starts.len()];
    let mut cursors: Vec<usize> = vec![0; starts.len()];

    while starts.iter().enumerate().any(|(i, c)| *c + cursors[i] < bytes[i].len() && bytes[i][*c + cursors[i]] != b' ') {
        for (i, s) in starts.iter().enumerate() {
            let c = bytes[i][*s + cursors[i]];
            if c == b' ' {
                cursors[i] += 1;
                continue;
            }

            let d = (c - b'0') as u64;
            terms[i] = (terms[i] * 10) + d;
            cursors[i] += 1;
        }
    }

    // Do one more iteration so we skip over the column of spaces
    (0..cursors.len()).for_each(|i| cursors[i] += 1);
    (cursors, terms)
}

fn parse_cephalopod(starts: &[usize], bytes: &[&[u8]]) -> (Vec<usize>, Vec<u64>) {
    let mut terms: Vec<u64> = vec![];
    let mut cursors: Vec<usize> = vec![0; starts.len()];

    while starts.iter().enumerate().any(|(i, c)| *c + cursors[i] < bytes[i].len() && bytes[i][*c + cursors[i]] != b' ') {
        let mut term = 0;
        for (i, s) in starts.iter().enumerate() {
            let c = bytes[i][*s + cursors[i]];
            if c == b' ' {
                cursors[i] += 1;
                continue;
            }

            let d = (c - b'0') as u64;
            term = (term * 10) + d;
            cursors[i] += 1;
        }

        terms.push(term);
    }

    // Do one more iteration so we skip over the column of spaces
    (0..cursors.len()).for_each(|i| cursors[i] += 1);
    (cursors, terms)
}

fn parse(input: &str, model: ParseModel) -> Vec<(Vec<u64>, Op)> {
    let lines: Vec<&str> = input.lines().collect();
    let line_count = lines.len();
    let term_count = line_count - 1; // count of term rows, not necessarily number of terms

    let operators: Vec<Op> = lines[term_count].split_whitespace()
        .map(|op| match op.as_bytes()[0] {
            b'*' => Op::Mul,
            b'+' => Op::Add,
            _ => panic!(),
        })
        .collect();

    let term_bytes: Vec<&[u8]> = lines.into_iter()
        .take(term_count)
        .map(|terms| terms.as_bytes())
        .collect();

    let mut term_cursors = vec![0; term_count];
    let mut problem_terms: Vec<Vec<u64>> = Vec::with_capacity(operators.len());
    while term_cursors[0] <= term_bytes[0].len() {
        let (cursors, terms) = match model {
            ParseModel::Human => parse_human(&term_cursors, &term_bytes),
            ParseModel::Cephalopod => parse_cephalopod(&term_cursors, &term_bytes),
        };

        (0..term_count).for_each(|i| term_cursors[i] += cursors[i]);
        problem_terms.push(terms);
    }

    problem_terms.into_iter().zip(operators).collect()
}

fn part_one(input: &str) -> u64 {
    parse(input, ParseModel::Human).into_iter()
        .map(|(terms, op)| match op {
            Op::Mul => terms.into_iter().product::<u64>(),
            Op::Add => terms.into_iter().sum::<u64>(),
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    parse(input, ParseModel::Cephalopod).into_iter()
        .map(|(terms, op)| match op {
            Op::Mul => terms.into_iter().product::<u64>(),
            Op::Add => terms.into_iter().sum::<u64>(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 4277556);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 3263827);
    }
}
