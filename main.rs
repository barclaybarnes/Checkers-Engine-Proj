// Constants representing the initial state of the board
const EMPTY: u64 = 0;
const PLAYER1: u64 = 0xAA55AA; // 101010100101010110101010 in binary
const PLAYER2: u64 = 0x55AA55 << 40; // 010101011010101001010101 in binary shifted to the top rows
const KING1: u64 = 0; // Kings for player 1
const KING2: u64 = 0; // Kings for player 2

// Function to print the current state of the board
fn print_board(player1: u64, player2: u64, king1: u64, king2: u64) {
    println!("  A B C D E F G H");
    for row in 0..8 {
        print!("{} ", 8 - row);
        for col in 0..8 {
            let pos = 1 << (row * 8 + col);
            if player1 & pos != 0 {
                if king1 & pos != 0 {
                    print!("B ");
                } else {
                    print!("b ");
                }
            } else if player2 & pos != 0 {
                if king2 & pos != 0 {
                    print!("W ");
                } else {
                    print!("w ");
                }
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

// Function to move a piece on the board
fn move_piece(player: &mut u64, king: &mut u64, opponent: &mut u64, opponent_king: &mut u64, from: usize, to: usize, is_player1: bool) -> bool {
    let from_mask = 1 << from; // Create a bitmask for the 'from' position
    let to_mask = 1 << to; // Create a bitmask for the 'to' position
    let row_diff = (to / 8) as isize - (from / 8) as isize;
    let col_diff = (to % 8) as isize - (from % 8) as isize;

    if *player & from_mask != 0 && *player & to_mask == 0 {
        if row_diff.abs() == 1 && col_diff.abs() == 1 {
            // Normal move
            *player &= !from_mask; // Clear the 'from' position
            *player |= to_mask; // Set the 'to' position
            if *king & from_mask != 0 {
                *king &= !from_mask; // Clear the 'from' position for king
                *king |= to_mask; // Set the 'to' position for king
            } else if (is_player1 && to / 8 == 0) || (!is_player1 && to / 8 == 7) {
                *king |= to_mask; // Promote to king if reaching the last row
            }
            return true;
        } else if row_diff.abs() == 2 && col_diff.abs() == 2 {
            // Jump-move
            let over = ((from as isize + to as isize) / 2) as usize;
            let over_mask = 1 << over; // Create a bitmask for the 'over' position
            if *opponent & over_mask != 0 {
                *player &= !from_mask; // Clear the 'from' position
                *player |= to_mask; // Set the 'to' position
                *opponent &= !over_mask; // Clear the 'over' position for opponent
                if *king & from_mask != 0 {
                    *king &= !from_mask; // Clear the 'from' position for king
                    *king |= to_mask; // Set the 'to' position for king
                    *opponent_king &= !over_mask; // Clear the 'over' position for opponent king
                } else if (is_player1 && to / 8 == 0) || (!is_player1 && to / 8 == 7) {
                    *king |= to_mask; // Promote to king if reaching the last row
                }
                return true;
            }
        }
    }
    false
}


// Function to capture an opponent's piece
fn capture_piece(player: &mut u64, king: &mut u64, opponent: &mut u64, opponent_king: &mut u64, from: usize, over: usize, to: usize, is_player1: bool) -> bool {
    let from_mask = 1 << from; // Create a bitmask for the 'from' position
    let over_mask = 1 << over; // Create a bitmask for the 'over' position
    let to_mask = 1 << to; // Create a bitmask for the 'to' position
    let row_diff = (to / 8) as isize - (from / 8) as isize; // Calculate the row difference
    let col_diff = (to % 8) as isize - (from % 8) as isize; // Calculate the column difference

    // Check if the move is valid for capturing
    if *player & from_mask != 0 && *opponent & over_mask != 0 && *player & to_mask == 0 && row_diff.abs() == 2 && col_diff.abs() == 2 {
        *player &= !from_mask; // Clear the 'from' position
        *player |= to_mask; // Set the 'to' position
        *opponent &= !over_mask; // Clear the 'over' position for opponent

        // Check if the piece being moved is a king
        if *king & from_mask != 0 {
            *king &= !from_mask; // Clear the 'from' position for king
            *king |= to_mask; // Set the 'to' position for king
            *opponent_king &= !over_mask; // Clear the 'over' position for opponent king
        } else if (is_player1 && to / 8 == 0) || (!is_player1 && to / 8 == 7) {
            *king |= to_mask; // Promote to king if reaching the last row
        }
        true // Return true indicating a successful capture
    } else {
        false // Return false if the move is not valid for capturing
    }
}


fn is_legal_move(player: u64, king: u64, opponent: u64, is_player1: bool) -> Vec<(String, String)> {
    let mut moves = Vec::new();
    for from in 0..64 {
        let from_mask = 1 << from; // Create a bitmask for the 'from' position
        if player & from_mask != 0 { // Check if the player has a piece at the 'from' position
            let directions = if king & from_mask != 0 {
                vec![-9, -7, 7, 9] 
            } else if is_player1 {
                vec![-9, -7] // Player 1 can move in the upward and downward diagonal directions
            } else {
                vec![7, 9] // Player 2 can move in the downward and upward diagonal directions
            };
            for &dir in &directions {
                let to = from as isize + dir;
                if to >= 0 && to < 64 {
                    let to = to as usize;
                    let to_mask = 1 << to; // Create a bitmask for the 'to' position
                    let from_col = (from % 8) as usize;
                    let to_col = (to % 8) as usize;
                    let from_row = (from / 8) as usize;
                    let to_row = (to / 8) as usize;
                    if from_col.abs_diff(to_col) == 1 && from_row.abs_diff(to_row) == 1 {
                        if player & to_mask == 0 && opponent & to_mask == 0 { // Check if the 'to' position is empty
                            if king & from_mask != 0 || (is_player1 && to_row < from_row) || (!is_player1 && to_row > from_row) {
                                moves.push((index_to_coord(from), index_to_coord(to))); // Add the move to the list of valid moves
                            }
                        }
                    }
                }
                // Check for jump moves
                let jump_to = from as isize + 2 * dir;
                let over = from as isize + dir;
                if jump_to >= 0 && jump_to < 64 && over >= 0 && over < 64 {
                    let jump_to = jump_to as usize;
                    let over = over as usize;
                    let jump_to_mask = 1 << jump_to; // Create a bitmask for the 'jump_to' position
                    let over_mask = 1 << over; // Create a bitmask for the 'over' position
                    let from_col = (from % 8) as usize;
                    let jump_to_col = (jump_to % 8) as usize;
                    let from_row = (from / 8) as usize;
                    let jump_to_row = (jump_to / 8) as usize;
                    if from_col.abs_diff(jump_to_col) == 2 && from_row.abs_diff(jump_to_row) == 2 {
                        if player & jump_to_mask == 0 && opponent & over_mask != 0 { // Check if the 'jump_to' position is empty and the 'over' position has an opponent piece
                            if king & from_mask != 0 || (is_player1 && jump_to_row < from_row) || (!is_player1 && jump_to_row > from_row) {
                                moves.push((index_to_coord(from), index_to_coord(jump_to))); // Add the jump move to the list of valid moves
                                // Recursively find further jumps
                                let mut further_jumps = is_legal_move(player | jump_to_mask, king, opponent & !over_mask, is_player1);
                                for (next_from, next_to) in further_jumps.iter_mut() {
                                    if next_from == &index_to_coord(jump_to) {
                                        *next_from = index_to_coord(from);
                                    }
                                }
                                moves.append(&mut further_jumps); // Add further jumps to the list of valid moves
                            }
                        }
                    }
                }
            }
        }
    }
    moves
}

// Allowing us to use Char Index spanning A-H and 1-8 to Move Pieces as would happen in a Real Engine
fn index_to_coord(index: usize) -> String {
    let col = (index % 8) as u8 + b'A';
    let row = 8 - (index / 8);
    format!("{}{}", col as char, row)
}

use std::io::{self, Write};
// User Input Function
fn get_user_input() -> (String, String) {
    let mut input = String::new();
    print!("Enter move (from to): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let from = parts[0].to_string();
    let to = parts[1].to_string();
    (from, to)
}

fn coord_to_index(coord: &str) -> usize {
    let col = coord.chars().next().unwrap() as usize - 'A' as usize;
    let row = 8 - coord.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
    row * 8 + col
}


//
fn update_game_state(player1: u64, player2: u64, player1_moves: u32, player2_moves: u32) -> bool {
    if player1 == 0 { // If Player 1 Has 0 Pieces Player Two Wins
        println!("Game over! Player 2 wins.");
        return false;
    } else if player2 == 0 { // If Player 2 Has 0 Pieces Player One Wins
        println!("Game over! Player 1 wins.");
        return false;
    } else if player1_moves > 30 || player2_moves > 30 { // If Moves per Player Exceeds this Game Ends (Working on Finding a Good Value (30 for Now))
        println!("Game over! Move limit exceeded.");
        return false;
    }
    true
}

fn main() {
    let mut player1 = PLAYER1;
    let mut player2 = PLAYER2;
    let mut king1 = KING1;
    let mut king2 = KING2;
    let mut current_player = 1;
    let mut player1_moves = 0;
    let mut player2_moves = 0;

    loop {
        println!("Player {}'s turn", current_player);
        print_board(player1, player2, king1, king2);
        println!("Player 1 moves: {}", player1_moves);
        println!("Player 2 moves: {}", player2_moves);
        let valid_moves = if current_player == 1 {
            is_legal_move(player1, king1, player2, true)
        } else {
            is_legal_move(player2, king2, player1, false)
        };
        println!("Valid moves: {:?}", valid_moves);
        let (from, to) = get_user_input();

        let from_index = coord_to_index(&from);
        let to_index = coord_to_index(&to);

        if current_player == 1 {
            if move_piece(&mut player1, &mut king1, &mut player2, &mut king2, from_index, to_index, true) {
                player1_moves += 1;
                current_player = 2;
            } else {
                println!("Invalid move. Try again.");
            }
        } else {
            if move_piece(&mut player2, &mut king2, &mut player1, &mut king1, from_index, to_index, false) {
                player2_moves += 1;
                current_player = 1;
            } else {
                println!("Invalid move. Try again.");
            }
        }

        if !update_game_state(player1, player2, player1_moves, player2_moves) {
            break;
        }
    }
}
