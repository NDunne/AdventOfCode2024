use shared::Solution;

mod shared;
mod solutions;

fn main() {
    assert!(
        std::env::args().len() == 2,
        "Expected: `AdventOfCode2024 <day number>`"
    );
    let day_number: u8 = std::env::args()
        .nth(1)
        .unwrap() // Safety: Assertion on args above
        .parse()
        .expect("Expect argument to be a number");

    let mut solver: Box<dyn Solution> = match day_number {
        1 => Box::new(solutions::day01::day01_solution::SolutionDay01::default()) as Box<dyn Solution>,
        2 => Box::new(solutions::day02::day02_solution::SolutionDay02::default()) as Box<dyn Solution>,
        3 => Box::new(solutions::day03::day03_solution::SolutionDay03::default()) as Box<dyn Solution>,
        _ => panic!("Unexpected day number {}", day_number),
    };

    match solver.run() {
        Ok((part_1, part_2)) => {
            println!("Day {}:", day_number);
            println!("Part1: {}", part_1.to_string());
            println!("Part2: {}", part_2.to_string());
        }
        Err(err) => eprintln!("Encountered error: {err:?}"),
    }
}
