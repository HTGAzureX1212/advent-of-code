use std::collections::HashMap;

use aocfw::Solution;

pub struct Year2021Day5;

impl<'a> Solution<'a> for Year2021Day5 {
    type ParsedT = impl Iterator<Item = Line> + Clone;
    type Part1OutputT = u64;
    type Part2OutputT = u64;

    fn parse(input: &'a str) -> Self::ParsedT {
        input
            .lines()
            .map(|line| {
                let mut coords = line.split(" -> ");
                let mut start_split = coords.next().unwrap().split(",");
                let start = Point {
                    x: start_split.next().unwrap().parse().unwrap(),
                    y: start_split.next().unwrap().parse().unwrap()
                };

                let mut end_split = coords.next().unwrap().split(",");
                let end = Point {
                    x: end_split.next().unwrap().parse().unwrap(),
                    y: end_split.next().unwrap().parse().unwrap()
                };

                Line {
                    start,
                    end
                }
            })
    }

    fn part_1(mut data: Self::ParsedT) -> Self::Part1OutputT {
        let mut points = HashMap::<(u64, u64), u16>::new();

        data.clone().filter(|line| line.start.x == line.end.x)
            .for_each(|line|{
                for x in line.start.x.min(line.end.x)..=line.start.x.max(line.end.x) {
                    for y in line.start.y.min(line.end.y)..=line.start.y.max(line.end.y) {
                        if let Some(count) = points.get(&(x, y)) {
                            points.insert((x, y), count + 1);
                        }
                        else {
                            points.insert((x, y), 0);
                        }
                    }
                }
            });

        data.filter(|line| line.start.y == line.end.y)
            .for_each(|line|{
                for x in line.start.x.min(line.end.x)..=line.start.x.max(line.end.x) {
                    for y in line.start.y.min(line.end.y)..=line.start.y.max(line.end.y) {
                        if let Some(count) = points.get(&(x, y)) {
                            points.insert((x, y), count + 1);
                        }
                        else {
                            points.insert((x, y), 0);
                        }
                    }
                }
            });

        points.iter().filter(|(_, count)| count > &&1).count() as u64
    }

    fn part_2(mut data: Self::ParsedT) -> Self::Part2OutputT {
        0
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point
}

#[derive(Debug)]
pub struct Point {
    x: u64,
    y: u64
}

aocfw::solve_fn!(fn solve; struct Year2021Day5; filename "input/day5"; year 2021; day 5);
