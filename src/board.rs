use crate::piece::{Color, Field, Piece};
use regex::Regex;
use std::fmt;

pub struct InvalidMoveError;

// Implement the Display trait for the custom error type to provide a human-readable error message
impl fmt::Display for InvalidMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Please input a valid move!")
    }
}

// The board is implemented as an 8x8 array where each field is of type Field.
pub struct Board {
    board: [[Field; 8]; 8],
    is_draw: bool,
    is_won: bool,
}

impl Board {
    // Creates a new board with pieces set in the initial chess positions.
    pub fn new() -> Self {
        let mut board = [[Field::Empty(Color::White); 8]; 8];

        // Set the colors of the fields
        for row in 0..8 {
            for col in 0..8 {
                let color = if (row + col) % 2 == 0 {
                    Color::White
                } else {
                    Color::Black
                };
                board[row][col] = Field::Empty(color);
            }
        }

        // Initialize black pieces
        for i in 0..8 {
            board[1][i] = Field::Occupied(Piece::Pawn(Color::Black));
        }
        board[0][0] = Field::Occupied(Piece::Rook(Color::Black));
        board[0][7] = Field::Occupied(Piece::Rook(Color::Black));
        board[0][1] = Field::Occupied(Piece::Knight(Color::Black));
        board[0][6] = Field::Occupied(Piece::Knight(Color::Black));
        board[0][2] = Field::Occupied(Piece::Bishop(Color::Black));
        board[0][5] = Field::Occupied(Piece::Bishop(Color::Black));
        board[0][3] = Field::Occupied(Piece::Queen(Color::Black));
        board[0][4] = Field::Occupied(Piece::King(Color::Black));

        // Initialize white pieces
        for i in 0..8 {
            board[6][i] = Field::Occupied(Piece::Pawn(Color::White));
        }
        board[7][0] = Field::Occupied(Piece::Rook(Color::White));
        board[7][7] = Field::Occupied(Piece::Rook(Color::White));
        board[7][1] = Field::Occupied(Piece::Knight(Color::White));
        board[7][6] = Field::Occupied(Piece::Knight(Color::White));
        board[7][2] = Field::Occupied(Piece::Bishop(Color::White));
        board[7][5] = Field::Occupied(Piece::Bishop(Color::White));
        board[7][3] = Field::Occupied(Piece::Queen(Color::White));
        board[7][4] = Field::Occupied(Piece::King(Color::White));

        Board {
            board,
            is_draw: false,
            is_won: false,
        }
    }

    // Checks if the given move is valid.
    // TODO: Update the function to handle actual move validation.
    fn is_valid_move(&self, input: &str) -> bool {
        let re = Regex::new(r"([BRQNK])?[a-h]?[1-8]?[x-]?[a-h][1-8](=[BRQN])?[+#]?").unwrap();

        if !re.is_match(input) {
            false
        } else {
            true
        }
    }

    // Updates the board state after a move.
    // TODO: Implement the function to handle making moves.
    pub fn next_move(&mut self, input: String) -> Result<(), InvalidMoveError> {
        if !self.is_valid_move(input.as_str()) {
            Err(InvalidMoveError)
        } else {
            Ok(())
        }
    }

    // Checks if the game has ended in a win or draw.
    pub fn is_over(&self) -> Option<bool> {
        self.is_draw.then(|| false).or(self.is_won.then(|| true))
    }

    // Prints the board with '|' between fields, including row numbers and column letters.
    pub fn get_position(&self) -> String {
        let mut result = String::new();

        for (i, row) in self.board.iter().enumerate() {
            result.push_str(&format!("{} |", 8 - i));

            for field in row {
                result.push_str(&format!("{} |", field.print()));
            }

            result.push('\n');
        }
        result.push_str("   A  B  C  D  E  F  G  H \n");

        result
    }
}
