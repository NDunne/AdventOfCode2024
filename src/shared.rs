use std::fmt;

pub struct Solution
{
    pub part1: isize,
    pub part2: isize
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Part 1: {} | Part 2: {}", self.part1, self.part2)
    }
}

pub type SolutionResult = anyhow::Result<Solution>;

pub trait Solver {    
    fn run<'a>(&mut self, lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> SolutionResult;
}
