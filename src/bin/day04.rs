use aoc2025::aoc;

const DAY: u32 = 4;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let first = input.lines().next().unwrap();
    let row_len = first.len() + 2; // 1 extra on either side

    let mut grid: Vec<Vec<u8>> = vec![
        std::iter::repeat_n(b'.', row_len).collect(),
    ];

    for line in input.lines() {
        let mut next = vec![b'.'];
        next.extend(line.as_bytes());
        next.push(b'.');
        grid.push(next);
    }

    grid.push(std::iter::repeat_n(b'.', row_len).collect());
    grid
}

fn accessible(grid: &[Vec<u8>], x: usize, y: usize) -> bool {
    if grid[y][x] == b'.' {
        return false;
    }

    let mut count = 0;
    count += i32::from(grid[y - 1][x - 1] == b'@');
    count += i32::from(grid[y - 1][x] == b'@');
    count += i32::from(grid[y - 1][x + 1] == b'@');
    count += i32::from(grid[y][x - 1] == b'@');
    count += i32::from(grid[y][x + 1] == b'@');
    count += i32::from(grid[y + 1][x - 1] == b'@');
    count += i32::from(grid[y + 1][x] == b'@');
    count += i32::from(grid[y + 1][x + 1] == b'@');

    count < 4
}

fn cartesian(max_x: usize, max_y: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..max_y).flat_map(move |y| (0..max_x).map(move |x| (x, y)))
}

fn remove(grid: &mut [Vec<u8>], x: usize, y: usize) -> bool {
    if accessible(grid, x, y) {
        grid[y][x] = b'.';
        return true;
    }

    false
}

fn remove_all(grid: &mut [Vec<u8>], max_x: usize, max_y: usize) -> usize {
    cartesian(max_x, max_y)
        .map(|(x, y)| remove(grid, x, y))
        .filter(|b| *b)
        .count()
}

fn part_one(input: &str) -> usize {
    let grid = parse(input);
    let max_y = grid.len();
    let max_x = grid[0].len();

    cartesian(max_x, max_y)
        .filter(|(x, y)| accessible(&grid, *x, *y))
        .count()
}

fn part_two(input: &str) -> usize {
    let mut grid = parse(input);
    let max_y = grid.len();
    let max_x = grid[0].len();

    let mut total = 0;
    let mut rolls: usize = remove_all(&mut grid, max_x, max_y);
    total += rolls;

    while rolls > 0 {
        rolls = remove_all(&mut grid, max_x, max_y);
        total += rolls;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&aoc::load_file("./examples", DAY)), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 43);
    }
}
