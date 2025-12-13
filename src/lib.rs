pub mod day1;
pub mod day2;
pub mod day3;

pub type Error = Box<dyn core::error::Error>;

#[derive(Debug, Clone, Copy)]
pub enum Part {
    Part1,
    Part2,
}
pub use Part::*;

pub trait Min {
    fn min() -> Self;
}
pub trait Max {
    fn max() -> Self;
}
macro_rules! impl_min_max {
    ($($t:ty),+) => {
        $(
            impl Min for $t {
                fn min() -> $t {
                    <$t>::MIN
                }
            }
            impl Max for $t {
                fn max() -> $t {
                    <$t>::MAX
                }
            }
        )+
    };
}
impl_min_max!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

