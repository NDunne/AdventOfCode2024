use std::{fmt, cmp};

use crate::shared::{FileReader, Solution};

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
enum ReportSafety
{
    Unsafe,
    Dampened,
    Safe
}

impl fmt::Display for ReportSafety {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReportSafety::Unsafe => write!(f, "Unsafe"),
            ReportSafety::Dampened => write!(f, "Dampened"),
            ReportSafety::Safe => write!(f, "Safe"),
        }
    }
}

const THRESHOLD: i32 = 3;

#[derive(Default)]
pub struct SolutionDay02 {
    is_safe: Vec<ReportSafety>
}

impl SolutionDay02
{
    fn determine_safe_pair(&self, v1: &i32, v2: &i32, direction: &mut i32) -> bool
    {
        let diff = v2 - v1;
        
        let abs_diff = diff.abs();
        let diff_sign = diff.checked_div(abs_diff).unwrap_or(0);

        let mut result = true;
        if abs_diff > THRESHOLD || abs_diff == 0 || diff_sign + *direction == 0
        {
            result = false
        }        
        else
        {
            *direction = diff_sign;
        }
        
        println!("{} {} ({}): {}", v1, v2, direction, result);
        result
    }

    fn determine_safe_windows<'a>(&self, row_item_windows: impl Iterator<Item = &'a [i32]>) -> ReportSafety
    {
        let mut direction = 0;
        let mut result = ReportSafety::Safe;
        
        for w in row_item_windows
        {
            println!("{:?}", w);
            match result {
                ReportSafety::Safe => {
                    if !self.determine_safe_pair(&w[0], &w[1], &mut direction)
                    {
                        if self.determine_safe_pair(&w[0], &w[2], &mut direction)
                        {
                            result = ReportSafety::Dampened;
                        }
                        else
                        {
                            result = ReportSafety::Unsafe;
                        }
                    }
                },
                ReportSafety::Dampened => {
                    if !self.determine_safe_pair(&w[1], &w[2], &mut direction)
                    {
                        result = ReportSafety::Unsafe;
                    }
                },
                ReportSafety::Unsafe => {
                    break;
                }
            }
        }
        result

    }

    fn determine_safe(&self, row_items: &Vec<i32>) -> ReportSafety
    {
        let safe_forward = self.determine_safe_windows(row_items.windows(3));
        let mut reverse_items = row_items.clone();
        reverse_items.reverse();
        let safe_reverse = self.determine_safe_windows(reverse_items.windows(3));

        let safe = *cmp::min(&safe_forward, &safe_reverse);

        println!("{} {} => {} {:?}", safe_forward, safe_reverse, safe, row_items);

        safe
    }
}

impl FileReader for SolutionDay02
{
    fn process_row(&mut self, row: &str) -> anyhow::Result<()>
    {
        let row_items: Vec<i32> = row.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();


        self.is_safe.push(self.determine_safe(&row_items));
        Ok(())
    }
}

impl Solution for SolutionDay02
{
    fn run(&mut self) -> anyhow::Result<(i32, i32)>
    {
        self.process_file("./src/solutions/day02/sample.txt")?;

        let result = self.is_safe.iter().fold((0, 0), |mut acc, safeness|{
            acc.0 += (safeness == &ReportSafety::Safe) as i32;
            acc.1 += (safeness != &ReportSafety::Unsafe) as i32;
            acc
        });

        Ok(result)
    }
}
