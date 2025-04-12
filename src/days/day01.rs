use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use crate::shared::{Solver, Solution, SolutionResult};

#[derive(Default)]
pub struct SolverDay01 {
    col1: Vec<isize>,
    col2: Vec<isize>,
}

impl Solver for SolverDay01
{
    fn run<'a>(&mut self, lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> SolutionResult {
        
        for line in lines
        {
            let cols = line.split("   ").collect::<Vec<&str>>();
            self.col1.push(cols[0].parse::<isize>()?);
            self.col2.push(cols[1].parse::<isize>()?);
        }
        self.col1.sort();
        self.col2.sort();

        let mut col2_counter = HashMap::new();
        
        for value in self.col2.iter() {
            match col2_counter.entry(value) {
                Occupied(mut e) => *e.get_mut() += 1,
                Vacant(e) => {
                    e.insert(1);
                }
            }
        }

        Ok(self.col1
            .iter()
            .zip(self.col2.iter())
            .fold(Solution::default(), |mut res: Solution, (value_1, value_2)| {
                res.part1 += (*value_2 - *value_1).abs();
                res.part2 += *value_1 * col2_counter.get(value_1).unwrap_or(&0);
                res
            })
        )
    }
}

#[test]
fn test_day1()
{
    let sample: Vec<&str> = vec![
        "3   4",
        "4   3",
        "2   5",
        "1   3",
        "3   9",
        "3   3"
    ];

    let mut solver = SolverDay01::default();
    let solution = solver.run(Box::new(sample.into_iter())).unwrap();

    assert_eq!(solution.part1, 11);
    assert_eq!(solution.part2, 31);
}

