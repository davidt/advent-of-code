use itertools::enumerate;
use std::fs::read_to_string;

#[derive(Clone, Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    pub fn turn(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }

    pub fn marker(&self) -> char {
        match self {
            Direction::NORTH => '^',
            Direction::EAST => '>',
            Direction::SOUTH => 'v',
            Direction::WEST => '<',
        }
    }
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Position {
    pub fn advance(
        &self,
        board: &Vec<Vec<char>>,
        w: usize,
        h: usize,
    ) -> Result<Position, &'static str> {
        let xi = self.x as i32;
        let yi = self.y as i32;

        let (x, y) = match self.direction {
            Direction::NORTH => (xi, yi - 1),
            Direction::EAST => (xi + 1, yi),
            Direction::SOUTH => (xi, yi + 1),
            Direction::WEST => (xi - 1, yi),
        };

        if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 {
            return Err("out of bounds");
        }

        let c = board[y as usize][x as usize];

        if c == '#' || c == '*' {
            if board[self.y][self.x] == self.direction.marker() {
                Err("loop")
            } else {
                let p = Position {
                    x: self.x,
                    y: self.y,
                    direction: self.direction.turn(),
                };
                return p.advance(board, w, h);
            }
        } else {
            Ok(Position {
                x: x as usize,
                y: y as usize,
                direction: self.direction.clone(),
            })
        }
    }
}

fn phase1(mut board: Vec<Vec<char>>, mut position: Position) {
    let h = board.len();
    let w = board[0].len();

    loop {
        match position.advance(&board, w, h) {
            Ok(new_position) => {
                board[position.y][position.x] = 'X';
                position = new_position;
            }
            Err(e) => {
                if e == "out of bounds" {
                    board[position.y][position.x] = 'X';
                    break;
                } else {
                    panic!();
                }
            }
        }
    }

    let mut total = 0;
    for line in board.iter() {
        for c in line.iter() {
            if c == &'X' {
                total += 1;
            }
        }
    }

    println!("{} distinct positions", total);
    println!();
}

fn is_loop(board: &mut Vec<Vec<char>>, w: usize, h: usize, mut position: Position) -> bool {
    loop {
        match position.advance(&board, w, h) {
            Ok(new_position) => {
                board[position.y][position.x] = position.direction.marker();
                position = new_position;
            }
            Err(e) => {
                if e == "loop" {
                    board[position.y][position.x] = position.direction.marker();
                    return true;
                } else if e == "out of bounds" {
                    return false;
                } else {
                    panic!();
                }
            }
        }
    }
}

fn test_for_loop(
    board: &mut Vec<Vec<char>>,
    w: usize,
    h: usize,
    position: Position,
    i: usize,
    j: usize,
) -> bool {
    board[i][j] = '*';

    return is_loop(board, w, h, position);
}

fn phase2(board: Vec<Vec<char>>, position: Position) {
    let h = board.len();
    let w = board[0].len();
    let mut total = 0;

    for i in 0..h {
        for j in 0..w {
            if board[i][j] != '.' {
                continue;
            }

            let result = test_for_loop(&mut board.clone(), w, h, position.clone(), i, j);

            if result {
                total += 1;
            }
        }
    }

    println!("{} obstructions cause loops", total);
}

fn main() {
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut position = Position {
        x: 0,
        y: 0,
        direction: Direction::NORTH,
    };

    for line in read_to_string("input").unwrap().lines() {
        board.push(line.chars().collect());
    }

    'position: for (y, line) in enumerate(board.iter()) {
        for (x, c) in enumerate(line.iter()) {
            if c == &'^' {
                position = Position {
                    x,
                    y,
                    direction: Direction::NORTH,
                };
                board[y][x] = '.';

                break 'position;
            }
        }
    }

    for line in board.iter() {
        println!("{}", String::from_iter(line));
    }
    println!();

    phase1(board.clone(), position.clone());
    phase2(board.clone(), position.clone());
}
