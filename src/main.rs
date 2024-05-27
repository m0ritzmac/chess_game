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
            if io::stdin().read_line(&mut input).is_ok() {
                match board.next_move(input.trim().to_string(), player.color()) {
                    Ok(_) => break,
                    Err(e) => println!("{}", e),
                }
            } else {
                println!("Failed to read input");
            }
        }

        // Print the updated board state
        println!("\n{}", board.get_position());

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
