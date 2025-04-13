use std::slice::Iter;
use std::fmt;

use itertools::Itertools;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::shared::{Solver, Solution, SolutionResult};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point
{
    x: usize,
    y: usize
}

#[derive(Debug, Copy, Clone, EnumIter, PartialEq, Eq)]
enum Direction
{
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

impl Point
{
    fn get_neighbour(&self, direction: Direction) -> Option<Point>
    {
        match direction {
            Direction::E  => Some(Point { x: self.x.checked_add(1)?, y: self.y     }),
            Direction::SE => Some(Point { x: self.x.checked_add(1)?, y: self.y.checked_add(1)? }),
            Direction::S  => Some(Point { x: self.x,                 y: self.y.checked_add(1)? }),
            Direction::SW => Some(Point { x: self.x.checked_sub(1)?, y: self.y.checked_add(1)? }),
            Direction::W  => Some(Point { x: self.x.checked_sub(1)?, y: self.y     }),
            Direction::NW => Some(Point { x: self.x.checked_sub(1)?, y: self.y.checked_sub(1)? }),
            Direction::N  => Some(Point { x: self.x,                 y: self.y.checked_sub(1)? }),
            Direction::NE => Some(Point { x: self.x.checked_add(1)?, y: self.y.checked_sub(1)? }),
        }
    }
}


struct LetterGrid<'a>
{
    grid: Vec<&'a str>
}

impl<'a> LetterGrid<'a>
{
    fn new(grid: Vec<&'a str>) -> Self
    {
        Self { grid }
    }

    fn iter(&'a self) -> Iter<'a, &'a str>
    {
        self.grid.iter()
    }

    fn get(&'a self, pos: &Point)  -> Option<char>
    {
        Some(self.grid.get(pos.y)?.chars().nth(pos.x)?)
    }

    fn find_all(&'a self, needle: &char) -> Vec<Point>
    {
        self.iter().enumerate().fold(Vec::<Point>::new(), |mut points, (row, s)| {
            points.extend(s.match_indices(*needle).map(|(idx, _)| Point { x: idx, y: row}));
            points
        })
    }
}

impl<'a> fmt::Display for LetterGrid<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter().join("\n"))
    }
}

pub struct SolverDay04 {}

impl SolverDay04
{
    fn find_all_word(wordsearch: &LetterGrid, word: &str) -> Vec<(Point, Direction)>
    {
        let candidate_origins = wordsearch.find_all( &word.chars().nth(0).unwrap());

        candidate_origins.iter().cartesian_product(Direction::iter()).filter_map(|(origin, direction)| {
            let mut next_point = origin.clone();

            for c in word[1..].chars()
            {
                next_point = next_point.get_neighbour(direction)?;
                wordsearch.get(&next_point).filter(|n| *n == c)?;
            }
            Some((origin.clone(), direction))
        }).collect()
    }
}

impl Solver for SolverDay04
{
    fn solve_impl<'a>(lines: Vec<&'a str>) -> SolutionResult
    {
        let wordsearch = LetterGrid::new(lines);
        let word = "XMAS";
       
        let mut result = Solution::default();
        result.part1 = Self::find_all_word(&wordsearch, word).len() as isize;
        
        Ok(result)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_point_get_neighbour()
    {
        let start = Point { x: 5, y: 7 };
        assert_eq!(start.get_neighbour(Direction::N),  Some(Point { x: 5, y: 6}));
        assert_eq!(start.get_neighbour(Direction::NE), Some(Point { x: 6, y: 6}));
        assert_eq!(start.get_neighbour(Direction::E),  Some(Point { x: 6, y: 7}));
        assert_eq!(start.get_neighbour(Direction::SE), Some(Point { x: 6, y: 8}));
        assert_eq!(start.get_neighbour(Direction::S),  Some(Point { x: 5, y: 8}));
        assert_eq!(start.get_neighbour(Direction::SW), Some(Point { x: 4, y: 8}));
        assert_eq!(start.get_neighbour(Direction::W),  Some(Point { x: 4, y: 7}));
        assert_eq!(start.get_neighbour(Direction::NW), Some(Point { x: 4, y: 6}));
        
        let origin = Point { x: 0, y: 0 };
        assert_eq!(origin.get_neighbour(Direction::SW), None);
        assert_eq!(origin.get_neighbour(Direction::W), None); 
        assert_eq!(origin.get_neighbour(Direction::NW), None);
        assert_eq!(origin.get_neighbour(Direction::N), None); 
        assert_eq!(origin.get_neighbour(Direction::NE), None);
    }

    #[test]
    fn test_find_all()
    {
        let input = LetterGrid::new(vec!["X..", ".X.", "..X"]);
    
        assert_eq!(input.find_all(&'X'), vec![Point { x: 0, y: 0 }, 
                                              Point { x: 1, y: 1 }, 
                                              Point { x: 2, y: 2}]);
    }

    #[test]
    fn test_grid_get()
    {
        let input = LetterGrid::new(vec!["ABC", "DEF", "GHI"]);
    
        assert_eq!(input.get(&Point { x: 0, y: 1 }).unwrap(), 'D');
        assert_eq!(input.get(&Point { x: 1, y: 0 }).unwrap(), 'B');
        assert_eq!(input.get(&Point { x: 1, y: 1 }).unwrap(), 'E');
        assert_eq!(input.get(&Point { x: 3, y: 3 }), None);
    }

    #[test]
    fn test_find_all_word()
    {
        let grid = vec!["BBB", "BAB", "BBB"];
        assert_eq!(SolverDay04::find_all_word(&LetterGrid::new(grid), "AB"), vec![
            (Point { x: 1, y: 1}, Direction::N), 
            (Point { x: 1, y: 1}, Direction::NE), 
            (Point { x: 1, y: 1}, Direction::E), 
            (Point { x: 1, y: 1}, Direction::SE), 
            (Point { x: 1, y: 1}, Direction::S), 
            (Point { x: 1, y: 1}, Direction::SW), 
            (Point { x: 1, y: 1}, Direction::W), 
            (Point { x: 1, y: 1}, Direction::NW), 
        ]);
    }

    #[test]
    fn test_sample()
    {
        let sample: &str = "
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
        ";

        let solution = SolverDay04::solve(Box::new(sample.split('\n'))).unwrap();
        assert_eq!(solution.part1, 18);
    }

}