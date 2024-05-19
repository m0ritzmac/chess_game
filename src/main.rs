mod board;
mod piece;
mod player;

use board::Board;
use player::Player;
use std::io;

fn main() {
    // Initialize the game board and set the starting player to White
    let mut board = Board::new();
    let mut player = Player::White;

    // Display welcome message and instructions
    println!("Welcome to chess");
    println!();
    println!("Please input your moves in shorthand notation, for example 'qxb1'.");

    loop {
        // Prompt the current player to input their move
        println!("Player {}, please input your move!", player.print());

        let mut input = String::new();

        // Read and validate the player's input
        loop {
            if io::stdin().read_line(&mut input).is_ok()
                && board.is_valid_move(input.trim().to_lowercase().as_str())
            {
                break;
            } else {
                println!("Your input was not a valid move, please try again!");
            }
        }

        // Update the board with the valid move
        board.next_move(input.trim().to_string().to_lowercase());

        // Print the updated board state
        println!();
        board.print();
        println!();

        // Check what do after every move.
        match board.is_over() {
            None => player.switch(),
            Some(win) => {
                if win {
                    println!("Player {} has won!", player.print());
                } else {
                    println!("The game ended in a draw");
                }
                break;
            }
        }
    }
}
