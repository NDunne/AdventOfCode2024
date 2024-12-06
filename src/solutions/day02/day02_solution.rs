use crate::shared::{FileReader, Solution};

#[derive(PartialEq, Eq)]
enum ReportSafety
{
    Unsafe,
    Dampened,
    Safe
}

#[derive(Default)]
pub struct SolutionDay02 {
    is_safe: Vec<ReportSafety>,
}

impl SolutionDay02
{
    fn determine_safe(&self, row_items: &Vec<i32>) -> anyhow::Result<ReportSafety>
    {
        let mut last_diff_sign = 0;

        for w in row_items.windows(2)
        {
            let diff = w[1] - w[0];
            if diff == 0
            {
                return Ok(ReportSafety::Unsafe);
            }

            let abs_diff = diff.abs();
            let diff_sign = diff / abs_diff;
            
            if abs_diff > 3 || last_diff_sign + diff_sign == 0
            {
                return Ok(ReportSafety::Unsafe);
            }

            last_diff_sign = diff_sign;            
        }
        Ok(ReportSafety::Safe)
    }
}

impl FileReader for SolutionDay02
{
    fn process_row(&mut self, row: &str) -> anyhow::Result<()>
    {
        let row_items = row.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        self.is_safe.push(self.determine_safe(&row_items)?);
        Ok(())
    }
}

impl Solution for SolutionDay02
{
    fn run(&mut self) -> anyhow::Result<(i32, i32)>
    {
        self.process_file("./src/solutions/day02/sample.txt")?;

        Ok((
            self.is_safe.iter().map(|x| (*x == ReportSafety::Safe) as i32).sum(),
            0
        ))
    }
}
