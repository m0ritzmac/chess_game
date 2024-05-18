use std::{fmt::Error, io};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    fn print(&self) -> char {
        match &self {
            Piece::Pawn(Color::Black) => '♙',
            Piece::Pawn(Color::White) => '♟',
            Piece::Knight(Color::Black) => '♘',
            Piece::Knight(Color::White) => '♞',
            Piece::Bishop(Color::Black) => '♗',
            Piece::Bishop(Color::White) => '♝',
            Piece::Rook(Color::Black) => '♖',
            Piece::Rook(Color::White) => '♜',
            Piece::Queen(Color::Black) => '♕',
            Piece::Queen(Color::White) => '♛',
            Piece::King(Color::Black) => '♔',
            Piece::King(Color::White) => '♚',
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Field {
    Empty(Color),
    Occupied(Piece),
}

impl Field {
    fn print(&self) -> char {
        match &self {
            &self::Field::Empty(Color::White) => '◻',
            &self::Field::Empty(Color::Black) => '◼',
            &self::Field::Occupied(piece) => piece.print(),
        }
    }
}

struct Move {
    piece: Piece, 
    field: usize,
}

struct Board {
    board: [[Field; 8]; 8],
}

impl Board {
    fn new() -> Self {
        let mut board = [[Field::Empty(Color::White); 8]; 8];

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

        // Initialize black pieces at the top of the board
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

        // Initialize white pieces at the bottom of the board
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

        Self { board }
    }

    fn print(&self) {
        for (i, row) in self.board.iter().enumerate() {
            print!("{} |", 8 - i); // Print row number

            for field in row {
                print!("{} |", field.print());
            }

            println!();
        }
        println!("   A  B  C  D  E  F  G  H ");
    }
}

fn main() {
    let board = Board::new();
    board.print();
}
