use crate::shared::{Solver, SolutionResult};

mod day01;
mod day02;

pub fn solve<'a>(day_number : u8, lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> SolutionResult
{
    match day_number {
                1 => day01::SolverDay01::solve(lines),
                2 => day02::SolverDay02::solve(lines),
                _ => Err(anyhow::anyhow!("Not Implemented"))
    }
}