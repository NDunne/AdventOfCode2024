
use crate::shared::{FileReader, Solution};

use regex::Regex;

#[derive(Default)]
pub struct SolutionDay03 {
    prodSum: i32
}

impl FileReader for SolutionDay03
{
    fn process_row(&mut self, row: &str) -> anyhow::Result<()>
    {
        let mul_matcher = Regex::new(r"mul\((\d+),(\d+)\)")?;

        self.prodSum += mul_matcher.captures_iter(row).fold(0, |mut prod, re_match|{
            println!("{} {:#?}", prod, re_match);
            let lhs = re_match[1].parse::<i32>().unwrap();
            let rhs = re_match[2].parse::<i32>().unwrap();
            prod += lhs * rhs;
            prod
        });

        Ok(())
    }
}

impl Solution for SolutionDay03
{
    fn run(&mut self) -> anyhow::Result<(i32, i32)>
    {
        self.process_file("./src/solutions/day03/input.txt")?;

        let result = (
            self.prodSum,
            0
        );

        Ok(result)
    }
}