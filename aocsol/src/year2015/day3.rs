use std::collections::{
    hash_map::Entry,
    HashMap
};

use aocfw::Solution;

pub struct Year2015Day3;

impl<'a> Solution<'a> for Year2015Day3 {
    type ParsedT = impl Iterator<Item = Direction> + Clone;
    type Part1OutputT = u64;
    type Part2OutputT = u64;

    fn parse(input: &'a str) -> Self::ParsedT {
        input
            .chars()
            .map(|c| {
                match c {
                    '>' => Direction::East,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    '^' => Direction::North,
                    _ => unreachable!()
                }
            })
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        let mut current = Coords(0, 0);
        let mut board = HashMap::new();

        *board.entry(current).or_insert(0) += 1;

        data.for_each(|direction| {
            current = update_coords(&board.entry(current), direction);
            *board.entry(current).or_insert(0) += 1;
        });

        board.len() as u64
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        let mut santa = Coords(0, 0);
        let mut robo_santa = Coords(0, 0);
        let mut board = HashMap::new();

        *board.entry(santa).or_insert(0) += 1;
        *board.entry(robo_santa).or_insert(0) += 1;

        let mut current = santa;

        data.for_each(|direction| {
            let state = update_coords(&board.entry(current), direction);

            *board.entry(state).or_insert(0) += 1;

            if current == santa {
                santa = state;
                current = robo_santa;
            }
            else {
                robo_santa = state;
                current = santa;
            }
        });

        board.len() as u64
    }
}

pub enum Direction {
    East,
    South,
    West,
    North
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coords(pub i64, pub i64);

fn update_coords(entry: &Entry<Coords, u64>, direction: Direction) -> Coords {
    let mut coords = *entry.key();

    match direction {
        Direction::East => coords.0 += 1,
        Direction::South => coords.1 -= 1,
        Direction::West => coords.0 -= 1,
        Direction::North => coords.1 += 1
    }

    coords
}

aocfw::solve_fn!(fn solve; struct Year2015Day3; filename "input/day3"; year 2015; day 3);