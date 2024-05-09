use clap::Parser;
use std::fmt;
use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const EMPTY: char = ' ';

#[derive(Copy, Clone, Eq, PartialEq)]
enum Player {
    X,
    O,
}

impl Player {
    fn toggle(&self) -> Player {
        match *self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn from_char(c: char) -> Option<Player> {
        match c {
            PLAYER_X => Some(Player::X),
            PLAYER_O => Some(Player::O),
            _ => panic!("Invalid player character"),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

struct Game {
    board: [[char; 3]; 3],
    current_player: Player,
}

impl Game {
    fn new(start_player: Player) -> Game {
        Game {
            board: [[EMPTY; 3]; 3],
            current_player: start_player, // X starts first
        }
    }

    fn play_move(&mut self, x: usize, y: usize) -> bool {
        if self.board[x][y] == EMPTY {
            self.board[x][y] = match self.current_player {
                Player::X => PLAYER_X,
                Player::O => PLAYER_O,
            };
            self.current_player = self.current_player.toggle();
            true
        } else {
            false
        }
    }

    fn check_winner(&self) -> Option<Player> {
        for i in 0..3 {
            // Check horizontal
            if self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
                && self.board[i][0] != EMPTY
            {
                return Some(Player::from_char(self.board[i][0])).flatten();
            }

            // Check vertical
            if self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
                && self.board[0][i] != EMPTY
            {
                return Some(Player::from_char(self.board[0][i])).flatten();
            }
        }
        // Check diagnoals
        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[0][0] != EMPTY
        {
            return Some(Player::from_char(self.board[0][0])).flatten();
        }
        if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
            && self.board[0][2] != EMPTY
        {
            return Some(Player::from_char(self.board[0][2])).flatten();
        }
        None
    }

    fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell != EMPTY))
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            writeln!(f, "{} | {} | {}", row[0], row[1], row[2])?;
        }
        Ok(())
    }
}

/// Tic-Tac-Toe Command Line Game
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Player to start the game, X or O
    #[clap(short, long, default_value = "X")]
    start_player: char,
}

fn main() {
    let args = Args::parse();
    let start_player = Player::from_char(args.start_player).unwrap_or(Player::X);
    let mut game = Game::new(start_player);
    println!("Starting the game!");
    println!("{}", game);

    let stdin = io::stdin();
    let mut input = String::new();

    // Interactive game loop
    while !game.is_full() && game.check_winner().is_none() {
        println!(
            "Player {}'s turn. Enter x, y coordinates for your move (0-2, 0-2):",
            game.current_player
        );

        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");

        // Attempt to split the input and parse as usize
        let coords: Vec<Option<usize>> = input
            .trim()
            .split(',')
            .map(|num| num.trim().parse::<usize>().ok())
            .collect();

        // Validate the coordinates
        if coords.len() == 2 && coords[0].is_some() && coords[1].is_some() {
            let x = coords[0].unwrap();
            let y = coords[1].unwrap();

            // Check the range of x and y
            if x > 2 || y > 2 {
                println!("Coordinates must be between 0 and 2. Please try again.");
                continue;
            }

            // Attempt to make a move
            if !game.play_move(x, y) {
                println!("Invalid move! Spot already taken or out of bounds, please try again.");
                continue;
            }
        } else {
            println!("Invalid input! Please enter the coordinates in the format x, y where both x and y are between 0 and 2.");
            continue;
        }

        println!("{}", game);

        // Check for a winner or a draw
        if let Some(winner) = game.check_winner() {
            println!("Player {} wins!", winner);
            break;
        }

        if game.is_full() {
            println!("It's a draw!");
            break;
        }
    }
}
