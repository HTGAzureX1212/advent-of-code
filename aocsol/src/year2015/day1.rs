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

pub fn run() {
    println!("Advent of Code 2015, Day 1");
    println!();

    print!("aocsol: stage 1: parsing input (year2015/input/day1)... ");
    let input = Year2015Day1::parse(include_str!("input/day1"));
    println!("done");

    println!();

    print!("aocsol: stage 2: computing answer for part 1... ");
    let solution = Year2015Day1::part_1(input);
    println!("done");
    println!("answer for part 1: {:?}", solution);

    println!();

    print!("aocsol: stage 3: computing answer for part 2... ");
    let solution = Year2015Day1::part_2(input);
    println!("done");
    println!("answer for part 2: {:?}", solution);
}
