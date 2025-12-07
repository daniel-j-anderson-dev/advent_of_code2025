use crate::{Part1, Part2, Puzzle};

const fn expected(part: Puzzle) -> usize {
    match part {
        Part1 => 18595663903,
        Part2 => todo!(),
    }
}
const INPUT: &str = include_str!("../input/day2.txt");

fn parse_input(input: &str) -> impl Iterator<Item = (usize, usize)> {
    input.split(',').filter_map(|range| {
        let mut range = range.split('-');
        let start = range.next()?.parse().ok()?;
        let end = range.next()?.parse().ok()?;
        Some((start, end))
    })
}

fn is_invalid(id: &usize) -> bool {
    let id = id.to_string();
    let len = id.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    let Some((left_half, right_half)) = id.split_at_checked(half) else {
        return false;
    };
    left_half == right_half
}

fn solution(part: Puzzle, input: &str) -> usize {
    // part1
    parse_input(input)
        .flat_map(|(start, end)| start..end)
        .filter(is_invalid)
        .sum()
}

#[test]
fn part1() {
    assert_eq!(expected(Part1), solution(Part1, INPUT));
}
