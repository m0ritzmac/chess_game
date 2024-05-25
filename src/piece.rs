use colored::*;

#[derive(Copy, Clone, Debug, PartialEq)]

// Every piece holds a color
pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    // Returns the color of the current piece
    pub fn color(&self) -> Color {
        match self {
            Piece::Pawn(color)
            | Piece::Rook(color)
            | Piece::Knight(color)
            | Piece::Bishop(color)
            | Piece::Queen(color)
            | Piece::King(color) => *color,
        }
    }

    // This functions prints the right unicode symbol for every peace
    pub fn print(&self) -> ColoredString {
        let dark_black = colored::Color::TrueColor { r: 0, g: 0, b: 0 };
        match &self {
            Piece::Pawn(Color::White) => "♙".white().bold(),
            Piece::Pawn(Color::Black) => "♟".color(dark_black),
            Piece::Knight(Color::White) => "♘".white().bold(),
            Piece::Knight(Color::Black) => "♞".color(dark_black),
            Piece::Bishop(Color::White) => "♗".white().bold(),
            Piece::Bishop(Color::Black) => "♝".color(dark_black),
            Piece::Rook(Color::White) => "♖".white().bold(),
            Piece::Rook(Color::Black) => "♜".color(dark_black),
            Piece::Queen(Color::White) => "♕".white().bold(),
            Piece::Queen(Color::Black) => "♛".color(dark_black),
            Piece::King(Color::White) => "♔".white().bold(),
            Piece::King(Color::Black) => "♚".color(dark_black),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]

// The color is either white or black
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]

// A field is either Empty or occupied by a piece.
pub enum Field {
    Empty(Color),
    Occupied(Piece),
}

impl Field {
    // This function prints the correct unicode symbol for Every Field
    pub fn print(&self) -> ColoredString {
        match self {
            Field::Empty(Color::White) => "◼".white(),
            Field::Empty(Color::Black) => "◻".black(),
            Field::Occupied(piece) => piece.print(),
        }
    }
}
