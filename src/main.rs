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

mod day5 {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl Point {
        // Turn string of format "1,2" into Point{1,2}
        fn from_string(s: &str) -> Point {
            let mut vals = s
                .split(",")
                .map(|s| -> usize { s.parse::<usize>().unwrap() });

            let x = vals.next().unwrap();
            let y = vals.next().unwrap();

            Point { x, y }
        }
    }

    #[derive(Debug)]
    struct Line {
        start: Point,
        end: Point,
    }

    impl Line {
        fn from_string(s: &str) -> Line {
            let mut iter = s.split(" -> ");
            let start = Point::from_string(iter.next().unwrap());
            let end = Point::from_string(iter.next().unwrap());
            Line { start, end }
        }

        fn is_horizontal(&self) -> bool {
            self.start.y == self.end.y
        }

        fn is_vertical(&self) -> bool {
            self.start.x == self.end.x
        }

        fn points(&self) -> Vec<Point> {
            let mut points = Vec::new();

            // only handle horizontal/vertical
            if self.is_horizontal() {
                let left = std::cmp::min(self.start.x, self.end.x);
                let right = std::cmp::max(self.start.x, self.end.x);
                for x in left..right + 1 {
                    points.push(Point { x, y: self.start.y })
                }
            } else if self.is_vertical() {
                let top = std::cmp::min(self.start.y, self.end.y);
                let bottom = std::cmp::max(self.start.y, self.end.y);
                for y in top..bottom + 1 {
                    points.push(Point { x: self.start.x, y })
                }
            } else {
                let x_op = if self.start.x < self.end.x {
                    |n: usize| n + 1
                } else {
                    |n: usize| n - 1
                };

                let y_op = if self.start.y < self.end.y {
                    |n: usize| n + 1
                } else {
                    |n: usize| n - 1
                };

                let mut x = self.start.x;
                let mut y = self.start.y;

                points.push(self.start.clone());

                while (x, y) != (self.end.x, self.end.y) {
                    x = x_op(x);
                    y = y_op(y);
                    points.push(Point { x, y });
                }
            }

            points
        }
    }

    pub fn day_5(input: &str) {
        let points = input
            .lines()
            .filter(|&s| !s.is_empty())
            .map(|line| -> Line { Line::from_string(line) })
            .collect::<Vec<_>>();

        let mut danger_points = HashMap::<Point, usize>::new();
        // Part 1 is only on horizontal vertical
        for line in points
            .iter()
            .filter(|&line| line.is_horizontal() || line.is_vertical())
        {
            for point in line.points() {
                (*danger_points.entry(point).or_insert(0)) += 1;
            }
        }

        let part1 = danger_points.iter().fold(0, |acc, (_, &count)| -> usize {
            if count >= 2 {
                return acc + 1;
            }
            return acc;
        });

        // Part two needs to add only the non-vertical non-horizontal lines
        for line in points
            .iter()
            .filter(|&line| !line.is_horizontal() && !line.is_vertical())
        {
            for point in line.points() {
                (*danger_points.entry(point).or_insert(0)) += 1;
            }
        }
        let part2 = danger_points.iter().fold(0, |acc, (_, &count)| -> usize {
            if count >= 2 {
                return acc + 1;
            }
            return acc;
        });

        println!("---Day5---");
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}

mod day6 {
    pub fn day_6(input: &str) {
        let mut fish = [0; 9];

        for n in input.split(',').map(|s| s.parse::<usize>().unwrap()) {
            fish[n] += 1;
        }

        for _i in 0..80 {
            let new_fish = fish[0];
            fish.rotate_left(1);
            fish[8] = new_fish;
            //fish[7] = 0;
            fish[6] += new_fish;
        }

        let part1: usize = fish.iter().sum();

        fish = [0; 9];
        for n in input.split(',').map(|s| s.parse::<usize>().unwrap()) {
            fish[n] += 1;
        }

        for _i in 0..256 {
            let new_fish = fish[0];
            fish.rotate_left(1);
            fish[8] = new_fish;

            fish[6] += new_fish;
        }
        let part2: usize = fish.iter().sum();

        println!("---Day 6---");
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    }
}

fn main() {
    println!("Hello, world!");
    day4::day_4(&fs::read_to_string("day4.txt").expect("Could not read file"));
    day5::day_5(&fs::read_to_string("day5.txt").expect("Could not read file"));
    day6::day_6(&fs::read_to_string("day6.txt").expect("Could not read file"));
}
