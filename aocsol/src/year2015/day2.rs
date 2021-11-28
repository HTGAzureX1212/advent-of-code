use std::cmp;

use aocfw::Solution;

pub struct Year2015Day2;

impl<'a> Solution<'a> for Year2015Day2 {
    type ParsedT = impl Iterator<Item = Dimensions> + Clone;
    type Part1OutputT = u64;
    type Part2OutputT = ();

    fn parse(input: &'a str) -> Self::ParsedT {
        input
            .lines()
            .map(|line| {
                let vec = line
                    .split("x")
                    .map(|int| int.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();

                Dimensions::from(vec)
            })
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        let mut total = 0;

        data.for_each(|dim| {
            total += 2 * dim.l * dim.w + 2 * dim.w * dim.h + 2 * dim.h * dim.l;
            total += cmp::min(cmp::min(dim.l * dim.w, dim.h * dim.w), dim.h * dim.l);
        });

        total
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
    }
}

pub struct Dimensions {
    l: u64,
    w: u64,
    h: u64
}

impl From<Vec<u64>> for Dimensions {
    fn from(vec: Vec<u64>) -> Self {
        Self {
            l: vec[0],
            w: vec[1],
            h: vec[2]
        }
    }
}

aocfw::solve_fn!(st Year2015Day2; filename "input/day2"; year 2015; day 2);