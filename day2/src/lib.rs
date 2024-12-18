use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    rows: Vec<Vec<ResultType>>,
}
impl Solution {
    fn add_row(&mut self, row: Vec<ResultType>) {
        self.rows.push(row);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            let row = line.split_whitespace().map(|v| v.parse().unwrap()).collect();
            solution.add_row(row);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for row in &self.rows {
            let max = row.iter().max().unwrap();
            let min = row.iter().min().unwrap();
            r += max - min;
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for row in &self.rows {
            'found: for a in row {
                for b in row {
                    if a > b && a%b==0 {
                        r += a / b;
                        break 'found;
                    }
                }
            }
        }
        // Implement for problem
        Ok(r)
    }
}
