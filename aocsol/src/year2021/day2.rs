use aocfw::Solution;

pub struct Year2021Day2;

impl<'a> Solution<'a> for Year2021Day2 {
    type ParsedT = impl Iterator<Item = Instruction> + Clone;
    type Part1OutputT = u64;
    type Part2OutputT = u64;

    fn parse(input: &'a str) -> Self::ParsedT {
        input.lines()
            .map(|line| {
                let mut split = line.split(" ");

                let instruction = split.next().unwrap();

                match instruction {
                    "forward" => {
                        Instruction::Forward(split.next().unwrap().parse().unwrap())
                    }
                    "up" => {
                        Instruction::Up(split.next().unwrap().parse().unwrap())
                    }
                    "down" => {
                        Instruction::Down(split.next().unwrap().parse().unwrap())
                    }
                    _ => unreachable!()
                }
            })
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        let mut horiz = 0;
        let mut depth = 0;

        data.for_each(|instruction| {
            match instruction {
                Instruction::Down(count) => depth += count,
                Instruction::Up(count) => depth -= count,
                Instruction::Forward(count) => horiz += count
            }
        });

        horiz * depth
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        let mut horiz = 0;
        let mut depth = 0;

        let mut aim = 0;

        data.for_each(|instruction| {
            match instruction {
                Instruction::Down(count) => {
                    aim += count;
                }
                Instruction::Up(count) => {
                    aim -= count;
                }
                Instruction::Forward(count) => {
                    horiz += count;
                    depth += aim * count;
                }
            }
        });

        horiz * depth
    }
}

pub enum Instruction {
    Down(u64),
    Forward(u64),
    Up(u64)
}

aocfw::solve_fn!(fn solve; struct Year2021Day2; filename "input/day2"; year 2021; day 2);
