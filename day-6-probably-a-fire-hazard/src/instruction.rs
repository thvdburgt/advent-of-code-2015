use crate::{GridPart1, GridPart2};

use regex::{Captures, Regex};
use std::str::FromStr;

enum InstructionType {
    Toggle,
    TurnOff,
    TurnOn,
}

pub struct Instruction {
    instruction_type: InstructionType,
    top_left: (usize, usize),
    bottom_right: (usize, usize),
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
}
impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction_type = if s.starts_with("toggle") {
            InstructionType::Toggle
        } else if s.starts_with("turn off") {
            InstructionType::TurnOff
        } else if s.starts_with("turn on") {
            InstructionType::TurnOn
        } else {
            return Err(());
        };

        let captures = RE.captures(s).unwrap();

        let get_capture = |captures: &Captures, i: usize| {
            captures
                .get(i)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .map_err(|_| ())
        };

        let top_left = (get_capture(&captures, 1)?, get_capture(&captures, 2)?);
        let bottom_right = (get_capture(&captures, 3)?, get_capture(&captures, 4)?);

        Ok(Self {
            instruction_type,
            top_left,
            bottom_right,
        })
    }
}

impl Instruction {
    pub fn follow_part_1(&self, grid: &mut GridPart1) {
        for x in self.top_left.0..=self.bottom_right.0 {
            for y in self.top_left.1..=self.bottom_right.1 {
                grid[y][x] = match self.instruction_type {
                    InstructionType::Toggle => !grid[y][x],
                    InstructionType::TurnOff => false,
                    InstructionType::TurnOn => true,
                }
            }
        }
    }

    pub fn follow_part_2(&self, grid: &mut GridPart2) {
        for x in self.top_left.0..=self.bottom_right.0 {
            for y in self.top_left.1..=self.bottom_right.1 {
                let lamp = &mut grid[y][x];
                *lamp = match self.instruction_type {
                    InstructionType::Toggle => *lamp + 2,
                    InstructionType::TurnOff => lamp.saturating_sub(1),
                    InstructionType::TurnOn => *lamp + 1,
                }
            }
        }
    }
}
