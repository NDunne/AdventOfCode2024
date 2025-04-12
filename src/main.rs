use std::path::PathBuf;

use std::fs::File;
use std::io::{BufRead, BufReader};

use matches::assert_matches;

mod shared;
mod days;

use shared::{Solution, SolutionResult};

fn get_solution(day_number : u8) -> SolutionResult
{
    let input_filepath = PathBuf::from(format!("./input/day{:02}.txt", day_number));

    let mut lines: Vec<String> = Vec::new();

    if let Ok(file) = File::open(&input_filepath)
    {
        lines = BufReader::new(file).lines().filter_map(Result::ok).collect();
    }
    let lines_iter = lines.iter().map(|s| s.as_str());
    days::solve(day_number, Box::new(lines_iter))
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

#[test]
fn verify_solutions()
{
    assert_matches!(get_solution(1), Ok(Solution { part1: 1830467, part2: 26674158 }));
    assert_matches!(get_solution(2), Ok(Solution { part1: 390, part2: 439 }));
}

