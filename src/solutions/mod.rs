use crate::shared::{Solver, SolutionResult};

mod day01;
mod day02;
mod day03;
mod day04;

pub fn solve<'a>(day_number : u8, lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> SolutionResult
{
    match day_number {
                1 => day01::SolverDay01::solve(lines),
                2 => day02::SolverDay02::solve(lines),
                3 => day03::SolverDay03::solve(lines),
                4 => day04::SolverDay04::solve(lines),
                _ => Err(anyhow::anyhow!("Not Implemented"))
    }
}