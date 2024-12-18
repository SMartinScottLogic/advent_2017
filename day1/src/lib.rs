use itertools::Itertools;
use std::io::{BufRead, BufReader};
#[allow(unused_imports)]
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    inputs: Vec<String>,
}
impl Solution {
    fn add_input(&mut self, line: String) {
        self.inputs.push(line);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            solution.add_input(line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for line in &self.inputs {
            r = 0;
            for (a, b) in line.chars().chain(line.chars().next()).tuple_windows() {
                if a == b {
                    r += a.to_digit(10).unwrap();
                }
            }
            debug!(r, line);
        }
        // Implement for problem
        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for line in &self.inputs {
            r = 0;
            for i in 0..line.len() {
                let a = line.chars().nth(i).unwrap();
                let b = line
                    .chars()
                    .nth((i + (line.len() >> 1)) % line.len())
                    .unwrap();
                if a == b {
                    r += a.to_digit(10).unwrap();
                }
            }
            info!(r, line);
        }
        // Implement for problem
        Ok(r as ResultType)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use tracing_test::traced_test;
    use utils::Solution;

    #[test]
    #[traced_test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
