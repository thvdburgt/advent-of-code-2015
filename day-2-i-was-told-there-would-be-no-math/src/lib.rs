use std::str::FromStr;

pub fn solve_puzzle_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|l: &str| {
            let present: Present = l.parse().unwrap();
            present.wrapping_paper_size()
        })
        .sum()
}

pub fn solve_puzzle_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|l: &str| {
            let present: Present = l.parse().unwrap();
            present.ribbon_length()
        })
        .sum()
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

#[derive(Debug)]
struct ParsePresentError;

impl Present {
    fn wrapping_paper_size(&self) -> u32 {
        self.area() + self.smallest_side_area()
    }

    fn ribbon_length(&self) -> u32 {
        let (a, b) = self.smallest_sides();
        a + a + b + b + self.volume()
    }

    fn area(&self) -> u32 {
        // Present(l, w, h) = self;
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn smallest_sides(&self) -> (u32, u32) {
        if self.l > std::cmp::max(self.w, self.h) {
            (self.w, self.h)
        } else {
            (self.l, std::cmp::min(self.w, self.h))
        }
    }

    fn smallest_side_area(&self) -> u32 {
        let (a, b) = self.smallest_sides();
        a * b
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

impl FromStr for Present {
    type Err = ParsePresentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dims: Vec<_> = s.split_terminator('x').collect();
        let l = dims[0].parse().map_err(|_| ParsePresentError)?;
        let w = dims[1].parse().map_err(|_| ParsePresentError)?;
        let h = dims[2].parse().map_err(|_| ParsePresentError)?;

        Ok(Present { l, w, h })
    }
}
