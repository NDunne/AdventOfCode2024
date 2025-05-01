use std::collections::{HashMap, HashSet};

use crate::shared::{Solver, Solution, SolutionResult};

#[derive(Default)]
struct Rule
{
    prohibited_before: HashSet<isize>,
    prohibited_after: HashSet<isize>
}

enum RuleResult
{
    Correct(isize),
    ReOrdered(isize)
}

#[derive(Default)]
struct RuleChecker {
    rules: HashMap<isize, Rule>
}

impl RuleChecker
{
    fn add_rule(&mut self, first: isize, second: isize)
    {
        println!("Add rule: {} -> {}", first, second);

        self.rules.entry(first).or_insert(Rule::default()).prohibited_before.insert(second);
        self.rules.entry(second).or_insert(Rule::default()).prohibited_after.insert(first);
    }

    fn check_rec(&self, update_slice: &[isize], prohibited: HashSet<isize>) -> Option<isize>
    {
        let pivot_idx = update_slice.len()/2;
        let pivot =  update_slice[pivot_idx];
        println!("Pivot: {}", pivot);
        println!("Prohibited: {:?}", prohibited);

        if prohibited.contains(&pivot)
        {
            return None;
        }

        let left = &update_slice[..pivot_idx];
        let mut left_prohibited = prohibited.clone();

        let right = &update_slice[pivot_idx+1..];
        let mut right_prohibited = prohibited;
        
        if let Some(pivot_rule) = self.rules.get(&pivot)
        {
            left_prohibited.extend(&pivot_rule.prohibited_before);
            right_prohibited.extend(&pivot_rule.prohibited_after);
        }

        println!("{:?} {:?}", left, right);

        if (left.len() == 0 || self.check_rec(left, left_prohibited).is_some()) &&
            (right.len() == 0 || self.check_rec(right, right_prohibited).is_some())
            {
                return Some(pivot);
            }
        None
    }

    fn check(&self, update: &Vec<isize>) -> Option<isize>
    {
        return self.check_rec(update, HashSet::new());
    }
}

pub struct SolverDay05 {

}

impl Solver for SolverDay05
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {
        let mut result = Solution::default();

        let mut rule_checker =  RuleChecker::default();

        for line in lines
        {
            if line.contains('|')
            {
                let rule_parts: Vec<isize> = line.split('|').map(|x| x.parse::<isize>().unwrap()).collect();
                rule_checker.add_rule(rule_parts[0], rule_parts[1]);
                continue;              
            }

            let update_parts: Vec<isize> = line.split(',').map(|x| x.parse::<isize>().unwrap()).collect();
            println!("Update: {:?}", update_parts);
            if let Some(correct) = rule_checker.check(&update_parts)
            {
                result.part1 += correct;
            }

        }


        Ok(result)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

        #[test]
    fn test_sample()
    {
        let sample: &str = "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47";
    
        let solution = SolverDay05::solve(Box::new(sample.split('\n'))).unwrap();
        assert_eq!(solution.part1, 143);

    }

}
