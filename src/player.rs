use crate::piece::Color;

// Enum representing a player, which can be either White or Black
pub enum Player {
    White,
    Black,
}

impl Player {
    // Switches the player to the opposite color
    pub fn switch(&mut self) {
        *self = match *self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };
    }

    // Returns the color associated with the current player
    pub fn color(&self) -> Color {
        match self {
            Player::White => Color::White,
            Player::Black => Color::Black,
        }
    }

    // Returns a string representation of the player's color
    pub fn print(&self) -> &str {
        match self {
            Player::White => "White",
            Player::Black => "Black",
        }
    }
}
