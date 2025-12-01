use aoc2025::aoc;

const DAY: u32 = 1;

fn main() {
    let fdata = aoc::load_file("data", DAY);

    println!("Part 1: {}", part_one(&fdata));
    println!("Part 2: {}", part_two(&fdata));
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Movement {
    dir: Direction,
    dist: u32,
}

fn parse(input: &str) -> Vec<Movement> {
    let mut movements: Vec<Movement> = vec![];

    for line in input.lines() {
        movements.push(Movement {
            dir: match line.chars().next().unwrap() {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("invalid direction"),
            },
            dist: line.get(1..).unwrap().parse::<u32>().unwrap(),
        });
    }

    movements
}

fn part_one(input: &str) -> u32 {
    let movements = parse(input);
    let mut position: i32 = 50;
    let mut password = 0;

    for movement in movements {
        match movement.dir {
            Direction::Left => position -= movement.dist as i32,
            Direction::Right => position += movement.dist as i32,
        }

        position %= 100;
        if position == 0 {
            password += 1;
        }
    }

    password
}

fn part_two(input: &str) -> u32 {
    let movements = parse(input);
    let mut position: i32 = 50;
    let mut password = 0;

    for movement in movements {
        let full_rotations: u32 = movement.dist / 100;
        let mid_rotation: i32 = (movement.dist % 100) as i32;
        let old_position: i32 = position;
        password += full_rotations;

        match movement.dir {
            Direction::Left => position -= mid_rotation,
            Direction::Right => position += mid_rotation,
        }

        // Don't double-count a turn off of 0.
        if old_position != 0 && (position <= 0 || position >= 100) {
            password += 1;
        }

        if position < 0 {
            position += 100;
        } else if position >= 100 {
            position -= 100;
        }
    }

    password
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
        assert_eq!(part_two(&aoc::load_file("./examples", DAY)), 6);
    }

    #[test]
    fn test_r1000() {
        assert_eq!(part_two("R1000"), 10);
    }
}
