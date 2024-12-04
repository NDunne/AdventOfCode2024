extern crate shared;

use shared::FileReader;

struct Day1FileReader
{
    col1: Vec<i32>,
    col2: Vec<i32>
}

impl FileReader for Day1FileReader
{
    fn process_row(&mut self, row: &str)
    {
        let cols = row.split("   ").collect::<Vec<&str>>();
        self.col1.push(cols[0].parse::<i32>().unwrap());
        self.col2.push(cols[1].parse::<i32>().unwrap());
    }
}

fn main() {
    let mut reader = Day1FileReader {col1: Vec::new(), col2: Vec::new()};

    let _result = reader.read_file("./Day1/input.txt");

    let mut col1 = reader.col1;
    let mut col2 = reader.col2;

    col1.sort();
    col2.sort();

    let mut diff_sum = 0;

    let mut sim_score = 0;
    let mut col2_count;

    for it in col1.iter().zip(col2.iter()) {
        let (v1, v2) = it;
        diff_sum += (*v1 - *v2).abs();
        
        col2_count = col2.iter().filter(|&n| *n == *v1).count() as i32;
        sim_score += *v1 * col2_count;
    }

    println!("Part1: {}", diff_sum);
    println!("Part2: {}", sim_score);
}
