# Crazy Checkers 🤪
A Small Checkers Engine with Jumps, King Conversions, and Suggested Moves


Game Rules:
- Player 1 is black in this case, in which the users would input a char combination of legal moves spanning from (A-H) and (1-8).
- Captures are made when a player jumps another player's piece in a diagonal pattern.
- King Conversions are made when a player's piece reaches the other side. King conversion is indicated by an uppercase letter.
- The game ends when players have either 0 pieces or exceeded 30 moves in the case of stalemate prevention.
- All pieces, including base pieces, can move backward and forward in a King Moveset fashion.

Process Taken:

I used countless programming resources, such as the Chessprograming Wiki, CodeScope's "Learn Rust and Build a Chess Computer" Guide, @neuroxofficial's Rust Bitboard Guide, and the Rust support forum.

Along with these, I used the Rust Programming Language Guide to learn various functionality and syntax, which are different from those of previous languages I've worked with.

Some of these new concepts included Vectors, Mutable Objects, Structs, Let Functionality, .abs, and Bit Shifting IE (<<).

I spent a lot of time researching previous users' chess engines built in Rust to give myself an idea of what a simple ASCII table bitboard game engine would look like in code within Rust.

Class Formatting:

As I'm new to using Rust and the RustRover IDE, I used a single .rs file to store all of my functionality, but it is split and explained with consistent documentation and comments.

Function Design

print_board: Prints the Board using all relevant boards (2-4) and the (A-H) and (1-8) index surrounding it. 

move_piece: This function creates bitmasks for 'to', 'from', and 'over' positions. This also handles King conversion, which is when a piece reaches the opposing end of the board.

capture_piece: This function handles bitmasks for jumping pieces and checks if the move is valid. Also indicates if a move is a successful capture or not.

is_legal_move: This function handles the 

Notes: I definitely should combine the move and capture piece functions when I have time, but it was easier to separate them and reuse code accordingly to reduce complexity at the time (It was late).






Bibliography
- [1]CodeScope, “Learn Rust and Build a Chesscomputer Ep.1: Board representation,” YouTube, Jul. 17, 2022. https://www.youtube.com/watch?v=0QaELGCt9WU (accessed Aug. 29, 2024).
- [2]J. Kreuzer, “Checker Bitboard Tutorial,” 3dkingdoms.com, 2024. https://3dkingdoms.com/checkers/bitboards.htm#bitboard (accessed Oct. 10, 2024).
- [3]“Writing a BitBoard in Rust Pt. 1: The Basics,” Nereuxofficials old Blog, 2021. https://nereuxofficial.github.io/posts/bitboard-rust/ (accessed Sep. 13, 2024).
- [4]“Board Representation - Chessprogramming wiki,” www.chessprogramming.org, Jan. 28, 2020. https://www.chessprogramming.org/Board_Representation (accessed Oct. 05, 2024).
- [5]The “Bitboard Game Help,” The Rust Programming Language Forum, Mar. 25, 2019. https://users.rust-lang.org/t/bitboard-game-help/26643/2 (accessed Oct. 09, 2024).
- [6]“Flipping Mirroring and Rotating - Chessprogramming wiki,” Chessprogramming.org, 2016. https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating (accessed Oct. 09, 2024).
- [7]“The Rust Programming Language - The Rust Programming Language,” doc.rust-lang.org. https://doc.rust-lang.org/book/title-page.html (accessed Sep. 18, 2024).
