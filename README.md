# angun-chess

**Board**
1. The board is a 2-d vector with String as elements.
   
2. The size is 8 by 8.
   
3. If there is no piece at a position it is replaced by three blank spaces "   ".to_string().
   
4. print_board(board: &Vec<Vec<String>>), prints the board to the terminal


**Pieces**
1. The pieces are orginally of the type &str but are converted to String when put inside the board.
   
2. They are separeated into two vectors as follows:
let black_pieces: Vec<&str> = vec![
        "bR1", "bK1", "bB1", "bKI", "bQU", "bB2", "bK2", "bR2", 
        "bP1", "bP2", "bP3", "bP4", "bP5", "bP6", "bP7", "bP8"
    ];
    
let white_pieces: Vec<&str> = vec![
        "wR1", "wK1", "wB1", "wKI", "wQU", "wB2", "wK2", "wR2", 
        "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8"
    ];
    
3. The pieces contain of three one characters long parts (except for the queen and king which have two parts):
   1. "w" or "b", representing the colors white and black.
   2. "R", "K", "B", "KI", "QU", "P", representing rook, knight, bishop, king, queen and pawn.
   3. "1", "2", ..., "8", representing the different pieces of the same type.
      
4. valid_pieces(whites_turn_to_move: bool, board: &Vec<Vec<String>>, is_in_check: bool) -> Vec<String>
   This function gives all the pieces that have valid moves for the given board and turnorder.
   
7. get_pieces_of_certain_color(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>
   This function gives all pieces remaining on the board that matches the color that the piece argument has.

**Indexing and searching**
1. Since the positions on a board is traditionally represented as "a1", "b1", "c2" and so on, but an index of the type (i32, i32)
   would be useful for indexing the board, there are functions for converting between the two.
     1. index_to_letter((i32, i32)) -> String
     2. letter_to_index(&str) -> (i32, i32)
        
2. There is also a method get_piece_at(board: &Vec<Vec<String>>, row: usize, col: usize) -> &String
   which returns the piece at a certain position
   
3. get_color(piece: &str) -> String gives the color "w" or "b" as a String
   
4. There are two functions for getting the position for a certain piece
   1. get_position_letter(piece: &str, board: &Vec<Vec<String>>) -> String, this returns the position as "d5".to_string() and similar
   2. get_position_index(piece: &str, board: &Vec<Vec<String>>) -> (i32, i32), gives the position viable for indexing through board
      
5. get_opposite_color(piece: &str) -> String, this function gives the opposite color of the piece argument.
   If the piece taken as input is black the function returns "w".to_string() and the opposite for a white piece.

**Moves**
1. valid_moves(piece_to_move: &str, board: &Vec<Vec<String>>) -> Vec<String>
   This function gives all the valid moves for a given piece and board position
   
2. king_moves(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>
   This function returns all the valid moves for the king in the given board. It is used in valid_moves()
   and is mostly a part of the valid_moves() function but could probably be used for only testing the kings moves.
   The function takes into consideration: the pieces moves according to chess rules, being within bounds,
   , not landing on a piece of the same color and not moving through any pieces. 
   The same function can be found for all other pieces but with king replaced with queen, rook, bishop, knight or pawn.

4. is_within_bounds(index: (i32, i32)) -> bool, this function gives true if the index is within (0,0) and (7,7)
   since those are the minimum and the maximum positions the piece can be in.

5. does_not_land_on_own(piece: &str, move_to: (i32, i32), board: &Vec<Vec<String>>) -> bool
   This function gives true if the position the piece intends to move to is not occupied by a piece of the same color as piece.

6. does_not_land_on_other(piece: &str, move_to: (i32, i32), board: &Vec<Vec<String>>) -> bool
   Same as does_not_land_on_own but for a piece of the opposite color.

**Check**
There are four methods used for handling check
1. is_in_check(piece: &str, board: &Vec<Vec<String>>) -> bool
   This function takes the color of piece and gives true if that color is currently in check.

2. solves_check_move(piece: &str, board: &Vec<Vec<String>>, mov: String) -> bool
   This function gives true if a specific move solves the current check position.
   The move could be "d4" meaning the given piece moves to d4.
   It uses is_in_check() before and after the move has been perfromed on a temporary board.

3. solves_check_piece(piece: &str, board: &Vec<Vec<String>>) -> bool
   This move tries all valid moves for a given piece and returns true if one of them solves check.

4. solves_check_all_pieces(piece: String, board: &Vec<Vec<String>>) -> Vec<String>
   This move tries all valid moves for all valid pieces and returns a vector with all the pieces that solves check.
   
**Other**
1. run_chess() can be used for testing visually but also for running a game of chess.




















