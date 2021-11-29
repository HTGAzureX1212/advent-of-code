use aocfw::Solution;

pub struct Year2015Day5;

impl<'a> Solution<'a> for Year2015Day5 {
    type ParsedT = impl Iterator<Item = &'a str> + Clone;
    type Part1OutputT = usize;
    type Part2OutputT = usize;

    fn parse(input: &'a str) -> Self::ParsedT {
        input.lines()
    }

    fn part_1(data: Self::ParsedT) -> Self::Part1OutputT {
        const VOWELS: &str = "aeiou";
        const NAUGHTY: [&str; 4] = ["ab", "cd", "pq", "xy"];

        data.filter(|s| {
            let vowels = s.chars()
                .filter(|c| VOWELS.contains(*c))
                .count() >= 3;
            let doubles = s.chars()
                .zip(s.chars().skip(1))
                .any(|(c1, c2)| c1 == c2);

            let naughty = NAUGHTY.iter().any(|pat| s.contains(pat));

            vowels && doubles && !naughty
        })
        .count()
    }

    fn part_2(data: Self::ParsedT) -> Self::Part2OutputT {
        data.filter(|s| {
            let double_doubles = s
                .chars()
                .zip(s.chars().skip(1))
                .zip(0..)
                .any(|(pair, i)| {
                    s.chars()
                        .skip(i + 2)
                        .zip(s.chars().skip(i + 3))
                        .find(|pair2| pair == *pair2)
                        .is_some()
                });

            let separated_doubles = s
                .chars()
                .zip(s.chars().skip(1))
                .zip(s.chars().skip(2))
                .any(|((c1, _), c3)| c1 == c3);

            double_doubles && separated_doubles
        })
        .count()
    }
}

aocfw::solve_fn!(fn solve; struct Year2015Day5; filename "input/day5"; year 2015; day 5);
