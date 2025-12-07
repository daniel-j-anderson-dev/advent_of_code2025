pub mod day1;
pub mod day2;

pub type Error = Box<dyn core::error::Error>;

#[derive(Debug, Clone, Copy)]
pub enum Puzzle {
    Part1,
    Part2
}
pub use Puzzle::*;
