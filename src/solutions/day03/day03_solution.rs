
use crate::shared::{FileReader, Solution};

use regex::Regex;

#[derive(Default)]
pub struct SolutionDay03 {
    enabled: bool,
    prod_sum: i32,
    conditional_prod_sum: i32
}

impl SolutionDay03
{
    fn process_str(&mut self, str: &str) -> anyhow::Result<()>
    {
        let mul_matcher = Regex::new(r"mul\((\d+),(\d+)\)")?;

        let chunk_prod_sum = mul_matcher.captures_iter(str).try_fold(0, |mut prod: i32, re_match| -> anyhow::Result<i32>
        {
            let lhs = re_match[1].parse::<i32>()?;
            let rhs = re_match[2].parse::<i32>()?;
            prod += lhs * rhs;
            Ok(prod)
        })?;

        self.prod_sum += chunk_prod_sum;
        if self.enabled
        {
            self.conditional_prod_sum += chunk_prod_sum;
        }

        Ok(())
    }
}

impl FileReader for SolutionDay03
{
    fn process_row(&mut self, row: &str) -> anyhow::Result<()>
    {
        for chunk in row.split("do")
        {
            if self.enabled && chunk.starts_with("n't()")
            {
                self.enabled = false;
            }
            if !self.enabled && chunk.starts_with("()")
            {
                self.enabled = true;
            }

            self.process_str(chunk)?;
        }

        Ok(())
    }
}

impl Solution for SolutionDay03
{
    fn run(&mut self) -> anyhow::Result<(i32, i32)>
    {
        self.enabled = true;
        self.process_file("./src/solutions/day03/input.txt")?;

        let result = (
            self.prod_sum,
            self.conditional_prod_sum
        );

        Ok(result)
    }
}