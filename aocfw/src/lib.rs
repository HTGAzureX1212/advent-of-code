use std::fmt::Debug;

pub trait Solution<'a> {
    type ParsedT: Clone;
    type Part1OutputT: Debug + PartialEq;
    type Part2OutputT: Debug + PartialEq;

    fn parse(input: &'a str) -> Self::ParsedT;
    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT;
    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT;
}
