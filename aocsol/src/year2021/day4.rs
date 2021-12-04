use aocfw::Solution;

pub struct Year2021Day4;

impl<'a> Solution<'a> for Year2021Day4 {
    type ParsedT = Game;
    type Part1OutputT = u64;
    type Part2OutputT = u64;

    fn parse(input: &'a str) -> Self::ParsedT {
        let mut paragraphs = input
            .trim()
            .split("\r\n\r\n");

        let numbers = paragraphs.next().unwrap();
        let numbers = numbers
            .split(',')
            .map(|n| {
                n.parse::<u64>().unwrap()
            })
            .collect::<Vec<_>>();

        let mut boards = Vec::new();
        for paragraph in paragraphs {
            let mut board = [[(0, false); 5]; 5];

            for (i, row) in paragraph.trim().lines().enumerate() {
                for (j, col) in row.trim().split_whitespace().map(|n| n.parse::<u64>().unwrap()).enumerate() {
                    board[i][j].0 = col;
                }
            }

            boards.push(Board(board));
        }

        Game {
            numbers,
            boards
        }
    }

    fn part_1(mut data: Self::ParsedT) -> Self::Part1OutputT {
        for number in &data.numbers {
            data.boards
                .iter_mut()
                .flat_map(|board| board.0.iter_mut())
                .flat_map(|row| row.iter_mut())
                .filter(|tuple| tuple.0 == *number)
                .for_each(|tuple| tuple.1 = true);

            if let Some(board) = data.boards.iter().find(|board| board_is_completed(board)) {
                return number * unmarked_total(board);
            }
        }

        unreachable!()
    }

    fn part_2(mut data: Self::ParsedT) -> Self::Part2OutputT {
        for number in &data.numbers {
            data.boards
                .iter_mut()
                .flat_map(|board| board.0.iter_mut())
                .flat_map(|row| row.iter_mut())
                .filter(|tuple| tuple.0 == *number)
                .for_each(|tuple| tuple.1 = true);

            if data.boards.len() > 1 {
                data.boards.retain(|board| !board_is_completed(board));
                
                continue;
            }

            if board_is_completed(&data.boards[0]) {
                return number * unmarked_total(&data.boards[0]);
            }
        }

        unreachable!()
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    numbers: Vec<u64>,
    boards: Vec<Board>
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Board([[(u64, bool); 5]; 5]);

fn board_is_completed(board: &Board) -> bool {
    let row = board.0.iter().any(|row| row.iter().all(|tile| tile.1));
    let col = (0..5).any(|col| board.0.iter().all(|row| row[col].1));

    row || col
}

fn unmarked_total(board: &Board) -> u64 {
    board.0
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| !tile.1)
        .map(|tile| tile.0)
        .sum()
}

aocfw::solve_fn!(fn solve; struct Year2021Day4; filename "input/day4"; year 2021; day 4);
