use aocfw::Solution;

pub struct Year2021Day1;

impl<'a> Solution<'a> for Year2021Day1 {
    type ParsedT = impl Iterator<Item = u64> + Clone;
    type Part1OutputT = usize;
    type Part2OutputT = usize;

    fn parse(input: &'a str) -> Self::ParsedT {
        input.lines()
            .map(|line| line.parse().unwrap())
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        data.clone()
            .zip(data.skip(1))
            .filter(|(v1, v2)| v1 < v2)
            .count()
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        data.clone()
            .zip(data.skip(3))
            .filter(|(v1, v2)| v1 < v2)
            .count()
    }
}

aocfw::solve_fn!(fn solve; struct Year2021Day1; filename "input/day1"; year 2021; day 1);
