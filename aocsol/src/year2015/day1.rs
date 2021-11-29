use aocfw::Solution;

pub struct Year2015Day1;

impl<'a> Solution<'a> for Year2015Day1 {
    type ParsedT = &'a str;
    type Part1OutputT = i64;
    type Part2OutputT = usize;

    fn parse(input: &'a str) -> Self::ParsedT {
        input
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        let mut floor = 0;

        data.chars()
            .for_each(|c| {
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

        for (i, c) in data.chars().enumerate() {
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
