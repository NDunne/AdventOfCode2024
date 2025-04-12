use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

use crate::shared::{Solver, Solution, SolutionResult};

#[derive(Default)]
pub struct SolverDay01 {
    col1: Vec<i32>,
    col2: Vec<i32>,
}

impl Solver for SolverDay01
{
    fn run<'a>(&mut self, lines: Box<dyn Iterator<Item = &'a str> + 'a>) -> SolutionResult {
        Ok(Solution { part1: 0, part2: 0 } )
    }
}

// impl FileReader for SolutionDay01 {
//     fn process_row(&mut self, row: &str) -> anyhow::Result<()> {
//         let cols = row.split("   ").collect::<Vec<&str>>();
//         self.col1.push(cols[0].parse::<i32>()?);
//         self.col2.push(cols[1].parse::<i32>()?);
//         Ok(())
//     }
// }

// impl Solution for SolutionDay01 {
//     fn run(&mut self) -> anyhow::Result<(i32, i32)> {
//         self.process_file("./src/solutions/day01/input.txt")?;
//         self.col1.sort();
//         self.col2.sort();

//         let mut col2_counter = HashMap::new();
//         for value in self.col2.iter() {
//             match col2_counter.entry(value) {
//                 Occupied(mut e) => *e.get_mut() += 1,
//                 Vacant(e) => {
//                     e.insert(1);
//                 }
//             }
//         }
//         let result =
//             self.col1
//                 .iter()
//                 .zip(self.col2.iter())
//                 .fold((0, 0), |mut acc, (value_1, value_2)| {
//                     acc.0 += (*value_2 - *value_1).abs();
//                     acc.1 += *value_1 * col2_counter.get(value_1).unwrap_or(&0);
//                     acc
//                 });
//         Ok(result)
//     }
// }
