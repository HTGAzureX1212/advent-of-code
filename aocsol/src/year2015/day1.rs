use aocfw::Solution;

pub struct Year2015Day1;

impl<'a> Solution<'a> for Year2015Day1 {
    type ParsedT = impl Iterator<Item = char> + Clone;
    type Part1OutputT = i64;
    type Part2OutputT = usize;

    fn parse(input: &'a str) -> Self::ParsedT {
        input.chars()
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        let mut floor = 0;

        data.for_each(|c| {
            if c == '(' {
                floor += 1;
            }
            else if c == ')' {
                floor -= 1;
            }
        });

        floor
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        let mut floor = 0;
        let mut index = 0;

        for (i, c) in data.enumerate() {
            if c == '(' {
                floor += 1;
            }
            else if c == ')' {
                floor -= 1;
            }

            if floor == -1 {
                index = i + 1;
                break;
            }
        }

        index
    }
}

aocfw::solve_fn!(fn solve; struct Year2015Day1; filename "input/day1"; year 2015; day 1);
