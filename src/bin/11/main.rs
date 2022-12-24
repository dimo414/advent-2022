use std::collections::VecDeque;
use std::str::FromStr;
use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;

use advent_2022::parsing::*;

fn main() -> Result<()> {
    let input = parse_input(include_str!("input.txt"))?;
    let mut game = KeepAway::new(&input, 3);
    game.rounds(20);
    println!("Monkey Business: {:?}", game.monkey_business());

    let mut game = KeepAway::new(&input, 1);
    game.rounds(10000);
    println!("Stressful Monkey Business: {:?}", game.monkey_business());

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Op,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    inspect_count: u64,
}

impl Monkey {
    fn handle(&mut self, relief_factor: u64) -> (usize, u64) {
        let item = self.items.pop_front().expect("Must have an item to handle");
        self.inspect_count += 1;
        let item = match self.operation {
            Op::Add(v) => item + v,
            Op::Mul(v) => item * v,
            Op::Square => item * item,
        };
        let item = item / relief_factor;
        let throw_to = if item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        };
        (throw_to, item)
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn make_op(op: &str) -> Result<Op> {
            if op == "old * old" {
                return Ok(Op::Square);
            }
            let regex = static_regex!(r"old ([+*]) (\d+)");
            let caps = regex_captures(regex, op)?;
            let op = capture_group(&caps, 1);
            let value = capture_group(&caps, 2).parse()?;

            match op {
                "+" => Ok(Op::Add(value)),
                "*" => Ok(Op::Mul(value)),
                _ => bail!("Invalid: {}", op),
            }
        }

        let regex = static_regex!(concat!(
            r"Monkey (\d+):\n",
            r"\s+Starting items: (.*)\n",
            r"\s+Operation: new = (old [+*] .*)\n",
            r"\s+Test: divisible by (\d+)\n",
            r"\s+If true: throw to monkey (.*)\n",
            r"\s+If false: throw to monkey (.*)"));
        let caps = regex_captures(regex, s)?;
        let id = capture_group(&caps, 1).parse()?;
        let items = capture_group(&caps, 2).split(", ")
            .map(|i| i.parse().context("")).collect::<Result<VecDeque<_>>>()?;
        let operation = make_op(capture_group(&caps, 3))?;
        let divisible_by = capture_group(&caps, 4).parse()?;
        let if_true = capture_group(&caps, 5).parse()?;
        let if_false = capture_group(&caps, 6).parse()?;
        Ok(Monkey{ id, items, operation, divisible_by, if_true, if_false, inspect_count: 0, })
    }
}

struct KeepAway {
    monkeys: Vec<Monkey>,
    relief_factor: u64,
}

impl KeepAway {
    fn new(monkeys: &[Monkey], relief_factor: u64) -> KeepAway {
        KeepAway { monkeys: monkeys.to_vec(), relief_factor, }
    }

    fn round(&mut self) {
        let lcd: u64 = self.monkeys.iter().map(|m| m.divisible_by).product();
        for m in 0..self.monkeys.len() {
            debug_assert_eq!(self.monkeys[m].id, m);
            while !self.monkeys[m].items.is_empty() {
                let (throw_to, item) = self.monkeys[m].handle(self.relief_factor);
                self.monkeys[throw_to].items.push_back(item % lcd);
            }
        }
    }

    fn rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.round();
        }
    }

    #[cfg(test)]
    fn items(&self) -> Vec<VecDeque<u64>> {
        self.monkeys.iter().map(|m| m.items.clone()).collect()
    }

    fn inspect_count(&self) -> Vec<u64> {
        self.monkeys.iter().map(|m| m.inspect_count).collect()
    }

    fn monkey_business(&self) -> u64 {
        self.inspect_count().iter().sorted().rev().take(2).product()
    }
}

fn parse_input(input: &str) -> Result<Vec<Monkey>> {
    input.split("\n\n").map(|m| m.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() { parse_input(include_str!("input.txt")).unwrap(); }

    #[test]
    fn example_1() {
        let monkeys = parse_input(include_str!("example.txt")).unwrap();
        let mut game = KeepAway::new(&monkeys, 3);
        game.round();
        assert_eq!(game.items(),
                   &[vec![20, 23, 27, 26], vec![2080, 25, 167, 207, 401, 1046], vec![], vec![]]);
        game.round();
        assert_eq!(game.items(),
                   &[vec![695, 10, 71, 135, 350], vec![43, 49, 58, 55, 362], vec![], vec![]]);
        // ...
        game.rounds(8);
        assert_eq!(game.items(),
                   &[vec![91, 16, 20, 98], vec![481, 245, 22, 26, 1092, 30], vec![], vec![]]);
        game.rounds(5);
        assert_eq!(game.items(),
                   &[vec![83, 44, 8, 184, 9, 20, 26, 102], vec![110, 36], vec![], vec![]]);
        game.rounds(5);
        assert_eq!(game.items(),
                   &[vec![10, 12, 14, 26, 34], vec![245, 93, 53, 199, 115], vec![], vec![]]);

        assert_eq!(game.inspect_count(), &[101, 95, 7, 105]);
        assert_eq!(game.monkey_business(), 10605);
    }

    #[test]
    fn example_2() {
        let monkeys = parse_input(include_str!("example.txt")).unwrap();
        let mut game = KeepAway::new(&monkeys, 1);

        game.round();
        assert_eq!(game.inspect_count(), &[2, 4, 3, 6]);
        game.rounds(19);
        assert_eq!(game.inspect_count(), &[99, 97, 8, 103]);
        game.rounds(980);
        assert_eq!(game.inspect_count(), &[5204, 4792, 199, 5192]);
        game.rounds(1000);
        assert_eq!(game.inspect_count(), &[10419, 9577, 392, 10391]);
        // ...
        game.rounds(8000);
        assert_eq!(game.inspect_count(), &[52166, 47830, 1938, 52013]);
        assert_eq!(game.monkey_business(), 2713310158);

    }
}
