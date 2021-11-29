use aocfw::Solution;
use crypto::{
    digest::Digest,
    md5::Md5,
};

pub struct Year2015Day4;

impl<'a> Solution<'a> for Year2015Day4 {
    type ParsedT = &'a [u8];
    type Part1OutputT = u64;
    type Part2OutputT = u64;

    fn parse(input: &'a str) -> Self::ParsedT {
        input.as_bytes()
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        let mut hasher = Md5::new();
        let mut counter = 0u64;

        loop {
            hasher.input(data);
            hasher.input(counter.to_string().as_bytes());

            let mut output = [0; 16];
            hasher.result(&mut output);

            let mut first = 0;

            for value in output.iter().take(5 / 2) {
                first += i32::from(*value);
            }

            first += i32::from(output[5 / 2] >> 4);

            if first == 0 {
                return counter;
            }

            hasher.reset();
            counter += 1;
        }
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        let mut hasher = Md5::new();
        let mut counter = 0u64;

        loop {
            hasher.input(data);
            hasher.input(counter.to_string().as_bytes());

            let mut output = [0; 16];
            hasher.result(&mut output);

            let mut first = 0;

            for value in output.iter().take(6 / 2) {
                first += i32::from(*value);
            }

            if first == 0 {
                return counter;
            }

            hasher.reset();
            counter += 1;
        }
    }
}

aocfw::solve_fn!(fn solve; struct Year2015Day4; filename "input/day4"; year 2015; day 4);
