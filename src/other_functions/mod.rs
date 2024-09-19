use std::io;
use crate::valid_moves::{self, is_in_check, solves_check_move, solves_check_all_pieces
};
const BOARD_SIZE: usize = 8;


//Used for tesing visually
pub fn run_chess(){

    //Create pieces
    let black_pieces: Vec<&str> = vec![
        "bR1", "bK1", "bB1", "bKI", "bQU", "bB2", "bK2", "bR2", 
        "bP1", "bP2", "bP3", "bP4", "bP5", "bP6", "bP7", "bP8"
    ];
    
    let white_pieces: Vec<&str> = vec![
        "wR1", "wK1", "wB1", "wKI", "wQU", "wB2", "wK2", "wR2", 
        "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8"
    ];

    //Set turn order
    let mut whites_turn_to_move = true;

    //Initilize board
    let mut board: Vec<Vec<String>> = vec![vec!["   ".to_string(); BOARD_SIZE]; BOARD_SIZE];
    for i in 0..8{
        board[0][i] = black_pieces.get(i)
        .expect("Piece at index {i} does not exist in white_pieces").to_string();
        board[1][i] = black_pieces.get(i + 8)
        .expect("Piece at index {i} does not exist in white_pieces").to_string();
    }

    for i in 0..8{
        board[7][i] = white_pieces.get(i)
        .expect("Piece at index {i} does not exist in white_pieces").to_string();
        board[6][i] = white_pieces.get(i + 8)
        .expect("Piece at index {i} does not exist in white_pieces").to_string();
    }
    

    //The chess-loop itself
    loop{   
        
        //Checking turn order
        if whites_turn_to_move {
            println!("\nWhites turn to move.");
        }
        else {
            println!("\nBlacks turn to move.");
        }

    
        //Checking which pieces can be moved
        let mut avalible_pieces_string: Vec<String> = Vec::new();
        let mut temp_piece = String::new();


        //Using temporary piece to use in functions
        //Its mostly to determine color
        if whites_turn_to_move {
            temp_piece.push_str("wKI");
        }
        else{
            temp_piece.push_str("bKI");
        }

        //Setting variable for check, to not have to use the function
        let mut is_in_check = false;
        if valid_moves::is_in_check(&temp_piece, &board) {
            is_in_check = true;
        }

        //Determining valid pieces based on check
        if is_in_check {
            avalible_pieces_string = valid_pieces(whites_turn_to_move, &board, true);
            if whites_turn_to_move {
                println!("      White is in Check!");
            }
            else {
                println!("      Black is in Check!");
            }
        }
        else{
            avalible_pieces_string = valid_pieces(whites_turn_to_move, &board, false);
        }

        let avalible_pieces: Vec<&str> = avalible_pieces_string.iter().map(|s| s.as_str()).collect();

        //Having checkmate here seemed easiest because avalible_pieces is close
        if avalible_pieces.is_empty() && whites_turn_to_move{
            println!("      Checkmate!");
            println!("      Black wins!");
            break;
        }
        else if avalible_pieces.is_empty() {
            println!("      Checkmate!");
            println!("      White wins!");
            break;
        }


        //Print out valid pieces
        println!("These are the avalible pieces:");
        for i in 0..avalible_pieces.len(){
            let to_print = avalible_pieces.get(i)
            .expect("Index out of bounds for vector avalible_pieces");
            print!("{to_print} ");
        }
    
    
        //Take input for which piece to move
        println!("\nWhich one do you want to move?"); 
        let mut input_piece = String::new();

        //Error handling for input_piece
        loop {
            input_piece.clear();       
    
        io::stdin().read_line(&mut input_piece).expect("Failed to read line.");
            
        let piece_inside_loop = input_piece.split_whitespace().collect::<String>();

        if !avalible_pieces.contains(&piece_inside_loop.as_str()) {
            println!("The answer you supplied does not match a valid piece. Please try again.");
        }
        else{
            break;
        }
        
        }
        let piece_to_move: String = input_piece.split_whitespace().collect::<String>();
        
        
    
        println!("These are the positions you can move it to: ");

        //If check -> filter out positions that do not solve check
        let mut moves = Vec::new();
        moves = valid_moves(piece_to_move.as_str(), &board);

        let mut moves_to_print = Vec::new();
        
        if is_in_check{
            moves_to_print = moves.iter().filter(|m| 
                solves_check_move(&piece_to_move, &board, m.to_string())).cloned().collect::<Vec<String>>();
        }
        else {
            moves_to_print = moves;
        }


        //Print valid positions
        let avalible_positions: Vec<&str> = moves_to_print.iter().map(|s| s.as_str()).collect();

        for i in 0..avalible_positions.len(){
            let to_print = avalible_positions.get(i)
            .expect("Index out of bounds for vector avalible_pieces");
            print!("{to_print} ");
        }
    
    
        //Take input on where to move the piece
        println!("\nWhere do you want to move?");
    
        let mut input_where_to = String::new();


        //Error handling for input_where_to
        loop {
            input_where_to.clear();       
    
        io::stdin().read_line(&mut input_where_to).expect("Failed to read line.");
            
        let position_inside_loop = input_where_to.split_whitespace().collect::<String>();

        if !moves_to_print.contains(&position_inside_loop) {
            println!("The answer you supplied does not match a position. Please try again.");
        }
        else{
            break;
        }
        
        }
    
        let where_to_move: &str = &input_where_to.split_whitespace().collect::<Vec<&str>>()[0];
    

        //Moving piece
        let move_from = get_position_index(piece_to_move.as_str(), &board);
        let move_to = letter_to_index(where_to_move);
        board[move_from.0 as usize][move_from.1 as usize] = "   ".to_string();
        board[move_to.0 as usize][move_to.1 as usize] = piece_to_move;    
    
    
        print_board(&board);

    
    
        //Change turns
        whites_turn_to_move = !whites_turn_to_move;

        
        

    }
    println!("GAME OVER");
    

}




pub fn get_color(piece: &str) -> String{
    let mut color = String::new();
    color.push_str(&piece[0..1]);
    color
}

pub fn get_position_index(piece: &str, board: &Vec<Vec<String>>) -> (i32, i32){
    letter_to_index(get_position_letter(piece, board).as_str())
}

pub fn get_position_letter(piece: &str, board: &Vec<Vec<String>>) -> String{
    let letters = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let mut position = String::from("");
    
    for row in 0..BOARD_SIZE {
        let col = &board[row];
        if let Some(index) = col.iter().position(|e| e == piece){
            let col_letter = letters[index];
            let row_number = 8 - row; 
            let row_letter = &row_number.to_string()[0..1];
            position = format!("{}{}", col_letter, row_letter);
            
        }
    }
    position
}

pub fn letter_to_index(place_to_move: &str) -> (i32, i32){
    let letters = ["a", "b", "c", "d", "e", "f", "g", "h"];

    let letter = &place_to_move[0..1];

    let col = letters.iter().position(|&x| x == letter)
    .expect("Value at this positions does not match a valid letter.");

    let number = &place_to_move[1..2];
    let row: i32 = BOARD_SIZE as i32 - number.parse::<i32>().expect("Failed to parse number");
    (row, col as i32)
}

pub fn index_to_letter(index: (i32, i32)) -> String{
    let mut index_to_letter = String::from("");
    let letters = ["a", "b", "c", "d", "e", "f", "g", "h"];

    let number = BOARD_SIZE as i32 - index.0;
    let number_as_string = &number.to_string()[0..1];
    let letter = letters[index.1 as usize];
    index_to_letter = format!("{}{}", letter, number_as_string);
    index_to_letter

}

pub fn valid_moves(piece_to_move: &str, board: &Vec<Vec<String>>) -> Vec<String> {

   let valid_positions = match piece_to_move {
       "wR1" | "wR2" | "bR1" | "bR2" => valid_moves::rook_moves(piece_to_move, board),
       "wB1" | "wB2" | "bB1" | "bB2" => valid_moves::bishop_moves(piece_to_move, board),
       "wK1" | "wK2" | "bK1" | "bK2" => valid_moves::knight_moves(piece_to_move, board),
       "wKI" | "bKI" => valid_moves::king_moves(piece_to_move, board),
       "wQU" | "bQU" => valid_moves::queen_moves(piece_to_move, board),
       _ => valid_moves::pawn_moves(piece_to_move, board),

   };

   valid_positions

}
    
pub fn print_board(board: &Vec<Vec<String>>){
    for i in 0..BOARD_SIZE{
        for j in 0..BOARD_SIZE{
            let to_print = &get_piece_at(&board, i, j);
            if j == BOARD_SIZE - 1{
                println!("{to_print}");
            }
            else {
                print!("{to_print}");
            }
        }
    }
}

pub fn valid_pieces(whites_turn_to_move: bool, board: &Vec<Vec<String>>, is_in_check: bool) -> Vec<String> { //De som har minst ett valid move
    let mut valid_pieces = Vec::new();

    if is_in_check{
        if whites_turn_to_move {
            for p in valid_moves::solves_check_all_pieces("wKI".to_string(), board){
                valid_pieces.push(p);
            }
        }
        else{
            for p in valid_moves::solves_check_all_pieces("bKI".to_string(), board){
                valid_pieces.push(p);
            }
        }
        
    }
    else {
        if whites_turn_to_move{
            for i in 0..BOARD_SIZE{
                for j in 0..BOARD_SIZE{
                    if !valid_moves(&get_piece_at(&board, i, j), board).is_empty()
                     && &board[i][j][0..1] == "w"{
                        valid_pieces.push(get_piece_at(&board, i, j).to_string());
                    } 
                }
            }
        }
        else{
            for i in 0..BOARD_SIZE{
                for j in 0..BOARD_SIZE{
                    if !valid_moves(get_piece_at(&board, i, j), board).is_empty()
                     && &board[i][j][0..1] == "b"{
                        valid_pieces.push(board[i][j].to_string());
                    } 
                }
            }
        }
    }
    
    

    valid_pieces
}

pub fn get_piece_at(board: &Vec<Vec<String>>, row: usize, col: usize) -> &String {
    &board[row][col] 
}


