use crate::shared::Solver;

mod day01;

pub fn get_solver<'a>(day_number : u8) -> Option<Box<dyn Solver + 'a>>
{
    match day_number {
                1 => Some(day01::SolverDay01::default()),
                _ => None
    }.map(|solver| Box::new(solver) as Box<dyn Solver>)
}