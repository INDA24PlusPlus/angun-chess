use std::io;
use crate::valid_moves;

const BOARD_SIZE: usize = 8;

pub fn run_chess(){
    //Format för index
    //row, col

    //Format för färg
    //Svart uppe, vit nere


    //Skapa svarta/vita pjäser 
    let black_pieces: Vec<&str> = vec![
        "bR1", "bK1", "bB1", "bKI", "bQU", "bB2", "bK2", "bR2", 
        "bP1", "bP2", "bP3", "bP4", "bP5", "bP6", "bP7", "bP8"
    ];
    
    let white_pieces: Vec<&str> = vec![
        "wR1", "wK1", "wB1", "wKI", "wQU", "wB2", "wK2", "wR2", 
        "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8"
    ];

    let mut whites_turn_to_move = true;

    let mut checkmate = false;
    

    loop{

        //Skapa board 
        //Detta måste också vara här för man ändrar på boarden
        
        let mut board: [[&str; BOARD_SIZE]; BOARD_SIZE] = [["   "; BOARD_SIZE]; BOARD_SIZE];
    
        for i in 0..8{
            board[0][i] = black_pieces.get(i)
            .expect("Piece at index {i} does not exist in white_pieces");
            board[1][i] = black_pieces.get(i + 8)
            .expect("Piece at index {i} does not exist in white_pieces");
        }
    
        for i in 0..8{
            board[7][i] = white_pieces.get(i)
            .expect("Piece at index {i} does not exist in white_pieces");
            board[6][i] = white_pieces.get(i + 8)
            .expect("Piece at index {i} does not exist in white_pieces");
        }
    
        //Användardialog + input ------------
    
        print_board(board);
        
        //Kollar turordning 
        if whites_turn_to_move {
            println!("\nWhites turn to move.");
        }
        else {
            println!("\nBlacks turn to move.");
        }
    
        //Kollar vilka pjäser som kan flyttas
        let avalible_pieces_string: Vec<String> = valid_pieces(whites_turn_to_move, board);
        let avalible_pieces: Vec<&str> = avalible_pieces_string.iter().map(|s| s.as_str()).collect();
    
        println!("These are the avalible pieces:");
        for i in 0..avalible_pieces.len(){
            let to_print = avalible_pieces.get(i)
            .expect("Index out of bounds for vector avalible_pieces");
            print!("{to_print} ");
        }
    
    
        
        println!("\nWhich one do you want to move?"); //Måste ha felhantering här, så man skriver en valid piece
        let mut input_which_one = String::new();
    
        io::stdin().read_line(&mut input_which_one).expect("Failed to read line.");
    
        let piece_to_move = input_which_one.split_whitespace().collect::<Vec<&str>>()[0];
    
        
    
        println!("These are the positions you can move it to: ");
    
        print_valid_moves(valid_moves(piece_to_move, board));
    
    
    
        println!("\nWhere do you want to move?");
    
        let mut input_where_to = String::new();
    
        io::stdin().read_line(&mut input_where_to).expect("Failed to read line.");
    
        let where_to_move: &str = &input_where_to.split_whitespace().collect::<Vec<&str>>()[0];
    
        //Slut användardialog + input ------------
    
    
        //Flytta pjäs -----------
    
        //Detta måste vara i huvudfunktionen pga
        //lånade värden (board) kan ej ändras
        let move_from = get_position_index(piece_to_move, board);
        let move_to = letter_to_index(where_to_move);
        board[move_from.0 as usize][move_from.1 as usize] = "   ";
        board[move_to.0 as usize][move_to.1 as usize] = piece_to_move;    
    
    
        print_board(board);

    
    
        //Ändra turordning
        whites_turn_to_move = !whites_turn_to_move;
        if checkmate {
            break;
        }

    }
    

}




pub fn get_color(piece: &str) -> String{
    let mut color = String::new();
    color.push_str(&piece[0..1]);
    color
}

pub fn get_position_index(piece: &str, board: [[&str; 8]; 8]) -> (i32, i32){
    letter_to_index(get_position_letter(piece, board).as_str())
}

pub fn get_position_letter(piece: &str, board: [[&str; 8]; 8]) -> String{
    let letters = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let mut position = String::from("");
    
    for row in 0..BOARD_SIZE {
        let col = board[row];
        if let Some(index) = col.iter().position(|&e| e == piece){
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

pub fn valid_moves(piece_to_move: &str, board: [[&str; 8]; 8]) -> Vec<String> {

   let color = get_color(piece_to_move);
   let position = get_position_index(piece_to_move, board);

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

fn print_valid_moves(valid_moves: Vec<String>){
    let avalible_positions: Vec<&str> = valid_moves.iter().map(|s| s.as_str()).collect();

    for i in 0..avalible_positions.len(){
        let to_print = avalible_positions.get(i)
        .expect("Index out of bounds for vector avalible_pieces");
        print!("{to_print}");
    }
}
    
fn print_board(board: [[&str; BOARD_SIZE]; BOARD_SIZE]){
    for i in 0..BOARD_SIZE{
        for j in 0..BOARD_SIZE{
            let to_print = board[i][j];
            if j == BOARD_SIZE - 1{
                println!("{to_print}");
            }
            else {
                print!("{to_print}");
            }
        }
    }
}

fn valid_pieces(whites_turn_to_move: bool, board: [[&str; 8]; 8]) -> Vec<String> { //De som har minst ett valid move
    let mut valid_pieces = Vec::new();

    if whites_turn_to_move{
        for i in 0..BOARD_SIZE{
            for j in 0..BOARD_SIZE{
                if !valid_moves(board[i][j], board).is_empty() && &board[i][j][0..1] == "w"{
                    valid_pieces.push(board[i][j].to_string());
                } 
            }
        }
    }
    else{
        for i in 0..BOARD_SIZE{
            for j in 0..BOARD_SIZE{
                if !valid_moves(board[i][j], board).is_empty() && &board[i][j][0..1] == "b"{
                    valid_pieces.push(board[i][j].to_string());
                } 
            }
        }
    }
    

    valid_pieces
}



