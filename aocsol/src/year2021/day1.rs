use aocfw::Solution;

pub struct Year2021Day1;

impl<'a> Solution<'a> for Year2021Day1 {
    type ParsedT = impl Iterator<Item = u64> + Clone;
    type Part1OutputT = u64;
    type Part2OutputT = u64;

    fn parse(input: &'a str) -> Self::ParsedT {
        input.lines()
            .map(|line| line.parse().unwrap())
    }

    fn part_1(mut data: Self::ParsedT) -> Self::Part1OutputT {
        let mut current  = data.next().unwrap();
        let mut increases = 0;

        while let Some(next) = data.next() {
            if next > current {
                increases += 1;
            }

            current = next;
        }

        increases
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        let depths = data.collect::<Vec<_>>();
        let mut windows = Vec::new();
        let mut increases = 0;

        for index in 2..depths.len() {
            windows.push(depths[index] + depths[index - 1] + depths[index - 2]);
        }

        for index in 1..windows.len() {
            if windows[index] > windows[index - 1] {
                increases += 1;
            }
        }

        increases
    }
}

aocfw::solve_fn!(fn solve; struct Year2021Day1; filename "input/day1"; year 2021; day 1);
