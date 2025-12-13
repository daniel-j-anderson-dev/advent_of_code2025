use crate::Min;

const INPUT: &str = include_str!("../input/day3.txt");

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

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = Battery>> {
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

#[test]
fn solution1() {
    let part1_solution = parse_input(INPUT)
        .map(|bank| {
            let [a, b] = two_largest(bank);
            let largest = if a.index < b.index { [a, b] } else { [b, a] };
            construct_two_digit_number(largest)
        })
        .sum::<usize>();

    assert!(part1_solution > 17236, "{} is not >\n17236", part1_solution);
}
