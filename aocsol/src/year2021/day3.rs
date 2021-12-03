use aocfw::Solution;

pub struct Year2021Day3;

impl<'a> Solution<'a> for Year2021Day3 {
    type ParsedT = impl Iterator<Item = Vec<char>> + Clone;
    type Part1OutputT = u64;
    type Part2OutputT = u64;

    fn parse(input: &'a str) -> Self::ParsedT {
        input
            .lines()
            .map(|line| line.chars().collect())
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        let mut epsilon = 0;
        let mut gamma = 0;

        (0..12).for_each(|i| {
            let mut ones = 0;
            let mut zeros = 0;

            data.clone().for_each(|vec| {
                if vec[i] == '0' {
                    zeros += 1;
                }
                else {
                    ones += 1;
                }
            });

            epsilon *= 2;
            gamma *= 2;

            if ones < zeros {
                gamma += 1;
            }
            else {
                epsilon += 1;
            }
        });

        epsilon * gamma
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        let mut oxygen = 0;
        let mut carbon_dioxide = 0;

        let mut oxygen_process = data.clone().collect::<Vec<_>>();
        for i in 0..12 {
            let mut ones = 0;
            let mut zeros = 0;

            for num in &oxygen_process {
                if num[i] == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }

            if oxygen_process.len() == 1 {
                break;
            }

            let filter_bit = if ones >= zeros { '1' } else { '0' };
            let done = oxygen_process.clone().into_iter().filter(|vec| vec[i] == filter_bit).collect::<Vec<_>>();
            oxygen_process = done.clone();

            if oxygen_process.len() == 1 {
                break;
            }
        }

        for i in 0..12 {
            oxygen *= 2;

            if oxygen_process[0][i] == '1' {
                oxygen += 1;
            }
        }

        let mut carbon_dioxide_process = data.clone().collect::<Vec<_>>();
        for i in 0..12 {
            let mut ones = 0;
            let mut zeros = 0;

            for num in &carbon_dioxide_process {
                if num[i] == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }

            if carbon_dioxide_process.len() == 1 {
                break;
            }

            let filter_bit = if zeros <= ones { '0' } else { '1' };
            let done = carbon_dioxide_process.clone().into_iter().filter(|vec| vec[i] == filter_bit).collect::<Vec<_>>();
            carbon_dioxide_process = done.clone();

            if carbon_dioxide_process.len() == 1 {
                break;
            }
        }

        for i in 0..12 {
            carbon_dioxide *= 2;

            if carbon_dioxide_process[0][i] == '1' {
                carbon_dioxide += 1;
            }
        }

        oxygen * carbon_dioxide
    }
}

aocfw::solve_fn!(fn solve; struct Year2021Day3; filename "input/day3"; year 2021; day 3);
