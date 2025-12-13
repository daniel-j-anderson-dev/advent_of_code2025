use crate::{Min, Part, Part1, Part2};

const INPUT: &str = include_str!("../input/day3.txt");
const fn expected(part: Part) -> usize {
    match part {
        Part1 => 17427,
        Part2 => todo!(),
    }
}

#[derive(Debug, Clone, Copy)]
struct Battery {
    index: usize,
    joltage: usize,
}
impl Battery {
    pub const MIN: Self = Self {
        index: usize::MIN,
        joltage: usize::MIN,
    };
}
impl Min for Battery {
    fn min() -> Self {
        Self::MIN
    }
}
impl PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        self.joltage == other.joltage
    }
}
impl PartialOrd for Battery {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.joltage.partial_cmp(&other.joltage)
    }
}

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = Battery> + Clone> {
    input.lines().map(|line| {
        line.bytes().enumerate().filter_map(|(index, b)| {
            Some(Battery {
                index,
                joltage: b.is_ascii_digit().then(|| (b - b'0') as _)?,
            })
        })
    })
}

const fn construct_two_digit_number([tens_place, ones_place]: [Battery; 2]) -> usize {
    assert!(tens_place.joltage < 10);
    assert!(ones_place.joltage < 10);
    tens_place.joltage * 10 + ones_place.joltage
}

fn two_largest<T: PartialOrd + Min>(iterator: impl Iterator<Item = T>) -> [T; 2] {
    iterator.fold([T::min(), T::min()], |[fst, snd], x| {
        if fst < x {
            [x, fst]
        } else if snd < x {
            [fst, x]
        } else {
            [fst, snd]
        }
    })
}

fn pairs<T: Copy>(i: impl Iterator<Item = T> + Clone) -> impl Iterator<Item = [T; 2]> {
    i.clone().flat_map(move |a| i.clone().map(move |b| [a, b]))
}

fn is_sorted_by_index(&[a, b]: &[Battery; 2]) -> bool {
    a.index < b.index
}

fn solution(part: Part, input: &str) -> usize {
    match part {
        Part1 => parse_input(input)
            .map(|bank| {
                pairs(bank)
                    .filter(is_sorted_by_index)
                    .map(construct_two_digit_number)
                    .max()
                    .unwrap_or_default()
            })
            .sum::<usize>(),
        Part2 => todo!(),
    }
}

#[test]
fn part1() {
    assert_eq!(solution(Part1, INPUT), expected(Part1));
}
