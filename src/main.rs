use std::path::PathBuf;

use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::anyhow;

mod shared;
mod days;

use shared::SolutionResult;
use days::get_solver;

fn get_solution(day_number : u8) -> SolutionResult
{
    let mut solver = get_solver(day_number).ok_or(anyhow!("Not Implemented"))?;
    let input_filepath = PathBuf::from(format!("./input/day{:02}.txt", day_number));

    let file = File::open(&input_filepath)?;
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(Result::ok).collect();
    let lines_iter = lines.iter().map(|s| s.as_str());
    solver.run(Box::new(lines_iter))
}

fn run_day(day_number : u8)
{
    match get_solution(day_number)
    {
        Ok(solution)  => println!("Day {:02}: {}", day_number, solution),
        Err(e) => eprintln!("Day {:02} : {}", day_number, e)
    }
}


fn main() 
{
    let day_number: Option<u8> = std::env::args().nth(1).map(|x| x.parse().expect("day argument must be number"));

    if let Some(selected_day) = day_number
    {
        run_day(selected_day);
    }
    else
    {
        for i in 1..26
        {
            run_day(i);
        }
    }


}
