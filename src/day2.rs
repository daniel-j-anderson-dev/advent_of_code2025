use crate::{Part, Part1, Part2};

const fn expected(part: Part) -> usize {
    match part {
        Part1 => 18595663903,
        Part2 => 19058204438,
    }
}

const INPUT: &str = include_str!("../input/day2.txt");

fn parse_input(input: &str) -> impl Iterator<Item = usize> {
    input
        .split(',')
        .filter_map(|range| {
            let (start, end) = range.split_once('-')?;
            let start = start.parse::<usize>().ok()?;
            let end = end.parse::<usize>().ok()?;
            Some(start..=end)
        })
        .flatten()
}

fn repeats_once<T: Eq>(id: impl AsRef<[T]>) -> bool {
    let id = id.as_ref();
    let length = id.len();

    if length % 2 != 0 {
        return false;
    }

    let (left, right) = id.split_at(length / 2);
    left == right
}

fn repeats_at_least_once<T: Eq>(id: impl AsRef<[T]>) -> bool {
    let id = id.as_ref();

    let mut patterns = (1..=id.len() / 2).filter_map(|pattern_length| {
        (id.len() % pattern_length == 0).then(|| &id[..pattern_length])
    });

    patterns.any(|pattern| {
        let mut chunks = (0..id.len() / pattern.len()).map(|chunk_index| {
            let start = chunk_index * pattern.len();
            let end = start + pattern.len();
            &id[start..end]
        });

        chunks.all(|chunk| chunk == pattern)
    })
}

fn solution(part: Part, input: &str) -> usize {
    parse_input(input)
        .filter(|id| {
            let id = id.to_string();
            match part {
                Part1 => repeats_once(id),
                Part2 => repeats_at_least_once(id),
            }
        })
        .sum()
}

#[test]
fn part1() {
    assert_eq!(expected(Part1), solution(Part1, INPUT));
}

#[test]
fn part2() {
    assert_eq!(expected(Part2), solution(Part2, INPUT));
}
