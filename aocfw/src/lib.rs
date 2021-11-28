#![feature(decl_macro)]

use std::fmt::Debug;

pub macro solve_fn {
    (st $st:ident; filename $filename:literal; year $year:literal; day $day:literal) => {
        pub fn solve() {
            println!("Advent of Code {}, Day {}", $year, $day);
            println!();

            print!("aocsol: stage 1: parsing input (year{}/input/day{})... ", $year, $day);
            let input = $st::parse(include_str!($filename));
            println!("done");

            println!();

            print!("aocsol: stage 2: computing answer for part 1... ");
            let solution = $st::part_1(input);
            println!("done");
            println!("answer for part 1: {solution:?}");

            println!();

            print!("aocsol: stage 3: computing answer for part 2... ");
            let solution = $st::part_2(input);
            println!("done");
            println!("answer for part 2: {solution:?}");
        }
    }
}

pub trait Solution<'a> {
    type ParsedT: Clone;
    type Part1OutputT: Debug + PartialEq;
    type Part2OutputT: Debug + PartialEq;

    fn parse(input: &'a str) -> Self::ParsedT;
    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT;
    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT;
}
