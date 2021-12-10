use std::fs;

mod day4 {
    #[derive(Debug)]
    struct Board {
        data: Vec<u32>,
        marks: Vec<bool>,
        size: usize,
    }

    impl Board {
        fn from_string(input: &[&str]) -> Board {
            let mut data = Vec::new();
            let mut size: Option<usize> = None;

            for row in input {
                let mut nums = row
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>();
                if size.is_none() {
                    size = Some(nums.len());
                } else {
                    assert!(size == Some(nums.len()));
                }

                data.append(&mut nums);
            }

            assert!(size == Some(data.len() / size.unwrap()));
            let data_len = data.len();

            Board {
                data: data,
                marks: vec![false; data_len],
                size: size.unwrap(),
            }
        }

        // return true if the board is a winner
        fn mark(&mut self, n: u32) {
            if let Some(idx) = self.data.iter().position(|&i| i == n) {
                self.marks[idx] = true;
            }
        }

        fn is_winner(&self) -> bool {
            // check for winning row
            for row in self.marks.chunks_exact(self.size) {
                if row.iter().all(|&i| i == true) {
                    return true;
                }
            }

            // check for winning column
            for col_start in 0..self.size {
                if self.marks[col_start..self.marks.len()]
                    .iter()
                    .step_by(self.size)
                    .all(|&i| i == true)
                {
                    return true;
                }
            }

            false
        }

        // get the sum of all unmarked numbers
        fn sum_unmarked(&self) -> u32 {
            self.data
                .iter()
                .zip(self.marks.iter())
                .filter(|(_, &marked)| !marked)
                .map(|(&n, _)| n)
                .fold(0, |acc, n| acc + n)
        }
    }

    pub fn day_4(input: &str) {
        println!("---Day4---");
        let lines = input.split('\n').collect::<Vec<&str>>();

        let numbers = lines
            .get(0)
            .unwrap()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let mut boards = Vec::new();

        for board_data in lines[1..lines.len()].chunks_exact(6) {
            let board = Board::from_string(&board_data[1..board_data.len()]);
            boards.push(board);
        }

        let mut first_winner: Option<u32> = None;
        let mut last_winner: Option<u32> = None;
        for num in numbers {
            let mut winners = Vec::new();
            for (i, board) in boards.iter_mut().enumerate() {
                board.mark(num);
                if board.is_winner() {
                    winners.push(i);
                    if first_winner.is_none() {
                        first_winner = Some(board.sum_unmarked() * num);
                    }
                    last_winner = Some(board.sum_unmarked() * num);
                }
            }

            for (i, &idx) in winners.iter().enumerate() {
                boards.remove(idx - i);
            }
        }

        println!("Part 1: {}", first_winner.unwrap());
        println!("Part 2: {}", last_winner.unwrap());
    }
}

mod day5 {}

fn main() {
    println!("Hello, world!");
    day4::day_4(&fs::read_to_string("day4.txt").expect("Could not read file"));
}
