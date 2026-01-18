// NOUGHTS AND CROSSES
use core::fmt;
use std::io;

fn get_user_input() -> Result<(usize, usize), std::num::ParseIntError> {
    println!("Enter x coordinate:");
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x: usize = x.trim().parse()?;

    println!("Enter y coordinate:");
    let mut y = String::new();
    io::stdin().read_line(&mut y).unwrap();
    let y: usize = y.trim().parse()?;
    Ok((x, y))
}



#[derive(Copy, Clone, PartialEq)]
enum Cell {
    X,
    O,
    Empty,
}

enum Player {
    X,
    O,
}


impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Cell::Empty => " ",
            Cell::X => "X",
            Cell::O => "O",
        };
        write!(f, "{}", symbol)
    }
}

struct Game {
    board: [[Cell; 3]; 3],
    turn: i32,
}

impl Game {
    fn show_board(&self) {
        println!("-------");
        for row in self.board {
            for cell in row {
                print!("|{}", cell);
            }
            print!("|");
            println!("\n-------");
        }
    }

    fn check_horizontal_and_vertical_matches(&self) -> Option<Player> {
        for x_idx in 0..3 {
            let mut horizontal = [Cell::Empty; 3];
            let mut vertical = [Cell::Empty; 3];
            for y_idx in 0..3 {
                horizontal[y_idx] = self.board[x_idx][y_idx];
                vertical[y_idx] = self.board[y_idx][x_idx];
            }
            if horizontal.iter().all(|&x| x == Cell::X) || vertical.iter().all(|&x| x == Cell::X) {
                return Some(Player::X)
                
            } else if horizontal.iter().all(|&x| x == Cell::O) || vertical.iter().all(|&x| x == Cell::O) {
                return Some(Player::O)
                
            }
        }
        None
    }
    
    fn check_diagonal_matches(&self) -> Option<Player> {
        let mut major_diagonal = [Cell::Empty; 3];
        let mut minor_diagonal = [Cell::Empty; 3];
        for idx in 0..3 {
            major_diagonal[idx] = self.board[idx][idx];
            minor_diagonal[idx] = self.board[idx][2 - idx];
        }
        if major_diagonal.iter().all(|&x| x == Cell::X) || minor_diagonal.iter().all(|&x| x == Cell::X) {
            return Some(Player::X)
        } else if major_diagonal.iter().all(|&x| x == Cell::O) || minor_diagonal.iter().all(|&x| x == Cell::O) {
            return Some(Player::O)
        }
        None
    }
}



fn main() {
    let mut game = Game{
        board: [[Cell::Empty; 3]; 3],
        turn: 0
    };
    println!("Lets play noughts and crosses!\n");
    println!(
        "Take turns entering coordinates until someone wins. Valid coordinates are in range [0, 2].\n"
    );
    loop {
        let (x, y) = match get_user_input() {
            Ok(coordinates) => coordinates,
            Err(_) => {
                println!("Invalid coordinate type, enter another...\n");
                continue;
            }
        };
        if x > 2 || y > 2 {
            print!("Invalid coordinate, enter another...\n");
            continue;
        }

        let existing_cell = game.board[y][x];
        if existing_cell != Cell::Empty {
            println!("Coordinate already exists, enter another...\n");
            continue;
        }

        if game.turn % 2 == 0 {
            game.board[y][x] = Cell::X;
        } else {
            game.board[y][x] = Cell::O;
        }
        game.show_board();
        match game.check_horizontal_and_vertical_matches() {
            Some(Player::X) =>  {
                println!("Player X is the winner!");
                break;
            },
            Some(Player::O) =>  {
                println!("Player O is the winner!");
                break;
            },
            None => {}
            
        };
        match game.check_diagonal_matches() {
            Some(Player::X) =>  {
                println!("Player X is the winner!");
                break;
            },
            Some(Player::O) =>  {
                println!("Player O is the winner!");
                break;
            },
            None => {}
            
        };

        game.turn += 1;
        if game.turn == 9 {
            println!("The game is a draw! you are both losers!!!")
        }
    }
}

