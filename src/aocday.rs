use std::fmt;
use std::fmt::Display;
use std::result;
use std::str;

use crate::DayError;

pub type DayResult<T> = result::Result<T, DayError>;
pub trait AoCDay: Sized {
	type Answer: PartialEq + Eq + fmt::Debug + Display;

	/// The display name of the puzzle. Ex: "Growing Pots"
	fn day() -> u8;
	fn name() -> &'static str;

	/// The parsing function. This should parse the input data into each day's own struct/data.
	fn parse(input: &str) -> DayResult<Self>;

	/// Part 1 implementation. T is for a direct value for tests, Box<dyn Display> is for displaying the result.
	fn part1(&mut self) -> DayResult<Self::Answer>;
	/// Part 2 implementation. T is for a direct value for tests, Box<dyn Display> is for displaying the result.
	/// If the result of part1 is needed for part2, then it should be stored in Self
	fn part2(&mut self) -> DayResult<Self::Answer>;
}

pub enum DayPart {
	Part1,
	Part2,
}
impl Display for DayPart {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DayPart::Part1 => write!(f, "Part 1"),
			DayPart::Part2 => write!(f, "Part 2"),
		}
	}
}
