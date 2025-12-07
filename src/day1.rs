use crate::{Part1, Part2, Puzzle};

const fn expected(part: Puzzle) -> i32 {
    match part {
        Part1 => 1152,
        Part2 => 6671,
    }
}
const INPUT: &str = include_str!("../input/day1.txt");
const DIAL_START: i32 = 50;
const DIAL_SIZE: i32 = 100;

fn apply_rotation(dial_position: i32, offset: i32) -> i32 {
    (dial_position + offset).rem_euclid(DIAL_SIZE)
}

fn floor_division(numerator: i32, denominator: i32) -> i32 {
    (numerator as f32 / denominator as f32).floor() as _
}

fn zero_crosses(dial_position: i32, offset: i32) -> i32 {
    let new_dial_position = dial_position + offset;
    if offset.is_negative() {
        floor_division(new_dial_position, -DIAL_SIZE) - floor_division(dial_position, -DIAL_SIZE)
    } else {
        floor_division(new_dial_position, DIAL_SIZE)
    }
}

fn solution(part: Puzzle, input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| match line.split_at_checked(1) {
            Some(("L", n)) => n.parse::<i32>().ok().map(|n| -n),
            Some(("R", n)) => n.parse().ok(),
            _ => None,
        })
        .fold((0, DIAL_START), |(zero_count, dial_position), offset| {
            let new_dial_position = (dial_position + offset).rem_euclid(DIAL_SIZE);
            let zero_count_offset = match part {
                Part1 => (new_dial_position == 0).into(),
                Part2 => zero_crosses(dial_position, offset),
            };
            (zero_count + zero_count_offset, new_dial_position)
        })
        .0
}

#[test]
fn part1() {
    assert_eq!(expected(Part1), solution(Part1, INPUT));
}
#[test]
fn part2() {
    assert_eq!(expected(Part2), solution(Part2, INPUT));
}
