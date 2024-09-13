use crate::other_functions::{self, get_color, get_position_index, index_to_letter, letter_to_index};

//Måste implementera schack regeln

pub fn queen_moves(piece: &str, board: [[&str; 8]; 8]) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);

    for i in 0..8{
        let changes_xy = vec![
        //Bishop moves
        (index.0 + i, index.1 - i), //x_plus, y_minus 
        (index.0 + i, index.1 + i), //x_plus, y_plus
        (index.0 - i, index.1 + i), //x_minus, y_plus
        (index.0 - i, index.1 - i), //x_minus, y_minus
        //Rook moves
        (index.0 - i, index.1), //x_minus
        (index.0 + i, index.1), //x_plus 
        (index.0, index.1 - i), //y_minus
        (index.0, index.1 + i),]; //y_plus 


        for j in changes_xy{
            if is_within_bounds(j) && does_not_collide(piece, j, board) {
                valid_moves.push(index_to_letter(j));
            }
        }
        
    }
    
    valid_moves
}

pub fn king_moves(piece: &str, board: [[&str; 8]; 8]) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = other_functions::get_position_index(piece, board);
    for row in index.0 - 1..index.0 + 2{
        for col in index.1 - 1..index.1 + 2{
            if is_in_check(piece, board) {
                if solves_check(piece, board) && (row, col) != (index.0, index.1) && is_within_bounds((row, col)) && does_not_collide(piece, (row, col), board){
                    valid_moves.push(index_to_letter((row, col)));
                }
            }
            else {
                if (row, col) != (index.0, index.1) && is_within_bounds((row, col)) && does_not_collide(piece, (row, col), board){
                    valid_moves.push(index_to_letter((row, col)));
                }
            }

        }
    }


    valid_moves
    
}

pub fn rook_moves(piece: &str, board: [[&str; 8]; 8]) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);

    for i in 0..8{
        let changes_xy = vec![
        (index.0 - i, index.1), //x_minus
        (index.0 + i, index.1), //x_plus 
        (index.0, index.1 - i), //y_minus
        (index.0, index.1 + i),]; //y_plus 


        for j in changes_xy{
            if is_within_bounds(j) && does_not_collide(piece, j, board) {
                valid_moves.push(index_to_letter(j));
            }
        }
        
    }
    
    valid_moves
}

pub fn bishop_moves(piece: &str, board: [[&str; 8]; 8]) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);

    for i in 0..8{
        let changes_xy = vec![
        (index.0 + i, index.1 - i), //x_plus, y_minus
        (index.0 + i, index.1 + i), //x_plus, y_plus
        (index.0 - i, index.1 + i), //x_minus, y_plus
        (index.0 - i, index.1 - i),]; //x_minus, y_minus


        for j in changes_xy{
            if is_within_bounds(j) && does_not_collide(piece, j, board) {
                valid_moves.push(index_to_letter(j));
            }
        }
        
    }
    
    valid_moves
}

pub fn knight_moves(piece: &str, board: [[&str; 8]; 8]) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);
    let changes_xy = vec![
    (index.0 + 2, index.1 - 1), //--^
    (index.0 + 2, index.1 + 1), //--v
    (index.0 - 2, index.1 - 1), //^--
    (index.0 - 2, index.1 + 1),//v--
    (index.0 + 1, index.1 + 2), //  |
                                //  |>

    (index.0 - 1, index.1 + 2), //  |
                                // <|

    (index.0 + 1, index.1 - 2), //  |>
                                //  |

    (index.0 - 1, index.1 - 2),];// <|
                                 //  |
    for j in changes_xy{
        if is_within_bounds(j) && does_not_collide(piece, j, board) {
            valid_moves.push(index_to_letter(j));
        }
    }
    valid_moves

}

pub fn pawn_moves(piece: &str, board: [[&str; 8]; 8]) -> Vec<String>{ 
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);
    let down = (index.0 + 1, index.1);
    let up = (index.0 - 1, index.1);
    let down_left = (index.0 + 1, index.1 - 1);
    let down_right = (index.0 + 1, index.1 + 1);
    let up_left = (index.0 - 1, index.1 - 1);
    let up_right = (index.0 - 1, index.1 + 1);
    
    if (get_color(piece).as_str() == "b") && is_within_bounds(down) && does_not_collide(piece, down, board) {
        valid_moves.push(other_functions::index_to_letter(down)); 

        if is_within_bounds(down_left) && does_not_collide(piece, down_left, board) &&
        get_color(board[down_left.0 as usize][down_left.1 as usize]) == "w" { 
            valid_moves.push(other_functions::index_to_letter(down_left)); 
        }
        else if is_within_bounds(down_right) && does_not_collide(piece, down_right, board) &&
        get_color(board[down_right.0 as usize][down_right.1 as usize]) == "w"  { 
            valid_moves.push(other_functions::index_to_letter(down_right)); 
        }
        
    }
    else if is_within_bounds(up) && does_not_collide(piece, up, board){
        valid_moves.push(other_functions::index_to_letter(up)); 

        if is_within_bounds(up_left) && does_not_collide(piece, up_left, board) && 
        get_color(board[up_left.0 as usize][up_left.1 as usize]) == "b" { 
            valid_moves.push(other_functions::index_to_letter(up_left)); 
        }
        else if is_within_bounds(up_right) && does_not_collide(piece, up_right, board) &&
        get_color(board[up_right.0 as usize][up_right.1 as usize]) == "b" { 
            valid_moves.push(other_functions::index_to_letter(up_right)); 
        }

        
    }

    valid_moves
    
}

fn is_within_bounds(index: (i32, i32)) -> bool {
    let max = (7,7);
    let min = (0,0);
    index.0 >= min.0 && index.0 <= max.0 && index.1 >= min.1 && index.1 <= max.1 
}

fn does_not_collide(piece: &str, move_to: (i32, i32), board: [[&str; 8]; 8]) -> bool {
    &board[move_to.0 as usize][move_to.1 as usize][0..1] != other_functions::get_color(piece).as_str()
}

//Om piece color = vit -> vitas drag
//Kolla vilken ruta vit kung är på
//Kolla alla svarta pjäsers valid moves
//Om vit kung ruta == valid move hos svart pjäs
//Vit är i schack
fn is_in_check(piece: &str, board: [[&str; 8]; 8]) -> bool{
    let pieces_to_try: Vec<String> = get_pieces_of_certain_color(&get_opposite_color(piece).as_str(), board);
    let mut king_position = String::new();
    let mut is_in_check = false;
    let mut king_of_certain_color = "";
    if get_color(piece).as_str() == "w"{
        king_of_certain_color = "wKI";
    }
    else{
        king_of_certain_color = "bKI";
    }
    king_position = other_functions::get_position_letter(king_of_certain_color, board);

    for p in pieces_to_try {
        for m in other_functions::valid_moves(&p, board){
            if m.as_str() == king_position.as_str(){
                is_in_check = true;
            }
        }
    }
    is_in_check

}

//Kolla is in schack på vit pjäs, om true -> vit är i schack och måste göra något
//För varje vit pjäs prova alla valid moves på en temp board
//Kolla is in schack på vit pjäs -> se om fortfarande i schack
//Om inte i schack, detta move solvar schack
fn solves_check(piece: &str, board: [[&str; 8]; 8]) -> bool {
    let mut solves_check = false;
    get_color(piece);
    let pieces = get_pieces_of_certain_color(piece, board);
    let mut temp_board = board;
    for p in pieces{
        let index_piece = get_position_index(piece, board);
        for mov in other_functions::valid_moves(piece, board) {
            temp_board[index_piece.0 as usize][index_piece.1 as usize] == "   ";
            let index_mov = letter_to_index(mov.as_str());
            temp_board[index_mov.0 as usize][index_mov.1 as usize] = mov.as_str();
            if !is_in_check(&p, temp_board) {
                solves_check = true;
            }
            temp_board = board;
        }
    }

    solves_check
}

fn get_pieces_of_certain_color(piece: &str, board: [[&str; 8]; 8]) -> Vec<String> {
    let pieces_to_check: Vec<String> = board.iter().flat_map(|row| row.iter())
    .filter(|&&x| x[0..1] == get_color(piece)).map(|&y| y.to_string()).collect();
    pieces_to_check
}

fn get_opposite_color(piece: &str) -> String{
    let mut opposite_color = String::new();
    if get_color(piece) == "w"{
        opposite_color.push_str("b");
    }
    else {
        opposite_color.push_str("w");
    }
    opposite_color
}
