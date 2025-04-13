use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;


mod shared;
mod solutions;

use shared::SolutionResult;

fn get_solution(day_number : u8) -> SolutionResult
{
    let input_filepath = PathBuf::from(format!("./input/day{:02}.txt", day_number));

    let mut lines: Vec<String> = Vec::new();

    if let Ok(file) = File::open(&input_filepath)
    {
        lines = BufReader::new(file).lines().filter_map(Result::ok).collect();
    }
    let lines_iter = lines.iter().map(|s| s.as_str());
    solutions::solve(day_number, Box::new(lines_iter))
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
    let first_arg: Option<String> = std::env::args().nth(1);

    if let Some(arg) = first_arg
    {
        if regex::Regex::new(r"^day\d\d?$").unwrap().is_match(&arg) {
            let selected_day = arg[3..].parse().unwrap();
            run_day(selected_day);
        }
        else
        {
            eprintln!("Pass a single argument 'dayXX', or no arguments to run all!");
            exit(1);
        }
    }
    else
    {
        for i in 1..26
        {
            run_day(i);
        }
    }
}

#[cfg(test)]
mod verify
{
    use super::*;
    use matches::assert_matches;
    use shared::Solution;

    #[test]
    fn day01()
    {
        assert_matches!(get_solution(1), Ok(Solution { part1: 1830467, part2: 26674158 }));
    }

    #[test]
    fn day02()
    {
        assert_matches!(get_solution(2), Ok(Solution { part1: 390, part2: 439 }));
    }

    #[test]
    fn day03()
    {
        assert_matches!(get_solution(3), Ok(Solution { part1: 187825547, part2: 85508223 }));
    }
    
    #[test]
    fn day04()
    {
        assert_matches!(get_solution(4), Ok(Solution { part1: 2578, part2: 1972 }));
    }
}

