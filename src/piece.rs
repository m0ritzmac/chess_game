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
    pub fn print(&self) -> char {
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
    pub fn print(&self) -> char {
        match self {
            Field::Empty(Color::White) => '◻',
            Field::Empty(Color::Black) => '◼',
            Field::Occupied(piece) => piece.print(),
        }
    }
}
