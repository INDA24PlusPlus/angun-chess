use crate::other_functions::{self, get_piece_at, get_position_index, index_to_letter, letter_to_index};





pub fn king_moves(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = other_functions::get_position_index(piece, board);
    for row in index.0 - 1..index.0 + 2{
        for col in index.1 - 1..index.1 + 2{
            if (row, col) != (index.0, index.1) && is_within_bounds((row, col)) 
            && does_not_land_on_own(piece, (row, col), board){
                valid_moves.push(index_to_letter((row, col)));
            }

        }
    }


    valid_moves
    
}


pub fn queen_moves(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);

    //Bishop
    let mut keep_checking_x_plus_y_minus = true;
    let mut keep_checking_x_plus_y_plus = true;
    let mut keep_checking_x_minus_y_plus = true;
    let mut keep_checking_x_minus_y_minus = true;

    //Rook
    let mut keep_checking_x_minus = true;
    let mut keep_checking_x_plus = true;
    let mut keep_checking_y_minus = true;
    let mut keep_checking_y_plus = true;

    let mut temp_bool = true;

    for i in 1..=8{
        let directions = vec![
        //Bishop
        (1, -1), //x_plus_y_minus
        (1, 1), //x_plus_y_plus 
        (-1, 1), //x_minus_y_plus
        (-1, -1), //x_minus_y_minus
        //Rook
        (-1, 0), //x_minus
        (1, 0), //x_plus 
        (0, -1), //y_minus
        (0, 1),]; //y_plus 


        for dir in directions{
            let move_to = (index.0 + dir.0*i, index.1 + dir.1*i); 
            if is_within_bounds(move_to){ //Får ej inehålla negativa värden i does_not_land_on_own
                if !does_not_land_on_own(piece, move_to, board) {
                    match dir {
                        //Bishop
                        (1, -1) => keep_checking_x_plus_y_minus = false,  
                        (1, 1) => keep_checking_x_plus_y_plus = false,  
                        (-1, 1) => keep_checking_x_minus_y_plus = false,  
                        (-1, -1) => keep_checking_x_minus_y_minus = false, 
                        //Rook
                        (-1, 0) => keep_checking_x_minus = false,  
                        (1, 0) => keep_checking_x_plus = false,  
                        (0, -1) => keep_checking_y_minus = false,  
                        _ => keep_checking_y_plus = false,       
                        };
                }
            }
            

            match dir {
                //Bishop
                (1, -1) => temp_bool = keep_checking_x_plus_y_minus,  
                (1, 1) => temp_bool = keep_checking_x_plus_y_plus,  
                (-1, 1) => temp_bool = keep_checking_x_minus_y_plus,  
                (-1, -1) => temp_bool = keep_checking_x_minus_y_minus, 
                //Rook
                (-1, 0) => temp_bool = keep_checking_x_minus,  
                (1, 0) => temp_bool = keep_checking_x_plus,  
                (0, -1) => temp_bool = keep_checking_y_minus,  
                _ => temp_bool = keep_checking_y_plus, 

                };
            

             
            if is_within_bounds(move_to) && does_not_land_on_own(piece, move_to, board) 
            && temp_bool {
                valid_moves.push(index_to_letter(move_to));
            }
                
            

            if is_within_bounds(move_to) {
                if !does_not_land_on_other(piece, move_to, board) {
                    match dir {
                        //Bishop
                        (1, -1) => keep_checking_x_plus_y_minus = false,  
                        (1, 1) => keep_checking_x_plus_y_plus = false,  
                        (-1, 1) => keep_checking_x_minus_y_plus = false,  
                        (-1, -1) => keep_checking_x_minus_y_minus = false, 
                        //Rook
                        (-1, 0) => keep_checking_x_minus = false,  
                        (1, 0) => keep_checking_x_plus = false,  
                        (0, -1) => keep_checking_y_minus = false,  
                        _ => keep_checking_y_plus = false,       
                        };
                }
            }
        }
        
    }
    
    valid_moves
}

pub fn rook_moves(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);

    let mut keep_checking_x_minus = true;
    let mut keep_checking_x_plus = true;
    let mut keep_checking_y_minus = true;
    let mut keep_checking_y_plus = true;
    let mut temp_bool = true;

    for i in 1..=8{
        let directions = vec![
        (-1, 0), //x_minus
        (1, 0), //x_plus 
        (0, -1), //y_minus
        (0, 1),]; //y_plus 


        for dir in directions{
            let move_to = (index.0 + dir.0*i, index.1 + dir.1*i); 
            if is_within_bounds(move_to){ 
                if !does_not_land_on_own(piece, move_to, board) {
                    match dir {
                        (-1, 0) => keep_checking_x_minus = false,  
                        (1, 0) => keep_checking_x_plus = false,  
                        (0, -1) => keep_checking_y_minus = false,  
                        _ => keep_checking_y_plus = false,       
                        };
                }
            }
            

            match dir {
                (-1, 0) => temp_bool = keep_checking_x_minus,  
                (1, 0) => temp_bool = keep_checking_x_plus,  
                (0, -1) => temp_bool = keep_checking_y_minus,  
                _ => temp_bool = keep_checking_y_plus,       
                };
            


            if is_within_bounds(move_to) && does_not_land_on_own(piece, move_to, board) //Maybe dont need this one
            && temp_bool {
                valid_moves.push(index_to_letter(move_to));
            }

            if is_within_bounds(move_to){ 
                if !does_not_land_on_other(piece, move_to, board) {
                    match dir {
                        (-1, 0) => keep_checking_x_minus = false,  
                        (1, 0) => keep_checking_x_plus = false,  
                        (0, -1) => keep_checking_y_minus = false,  
                        _ => keep_checking_y_plus = false,       
                        };
                }
            }
        }
        
    }
    
    valid_moves
}

pub fn bishop_moves(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>{
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);

    let mut keep_checking_x_plus_y_minus = true;
    let mut keep_checking_x_plus_y_plus = true;
    let mut keep_checking_x_minus_y_plus = true;
    let mut keep_checking_x_minus_y_minus = true;
    let mut temp_bool = true;

    for i in 1..=8{
        let directions = vec![
        (1, -1), //x_plus_y_minus
        (1, 1), //x_plus_y_plus 
        (-1, 1), //x_minus_y_plus
        (-1, -1),]; //x_minus_y_minus


        for dir in directions{
            let move_to = (index.0 + dir.0*i, index.1 + dir.1*i); 
            if is_within_bounds(move_to){ //Får ej inehålla negativa värden i does_not_land_on_own
                if !does_not_land_on_own(piece, move_to, board) {
                    match dir {
                        (1, -1) => keep_checking_x_plus_y_minus = false,  
                        (1, 1) => keep_checking_x_plus_y_plus = false,  
                        (-1, 1) => keep_checking_x_minus_y_plus = false,  
                        _ => keep_checking_x_minus_y_minus = false,       
                        };
                }
            }
            

            match dir {
                (1, -1) => temp_bool = keep_checking_x_plus_y_minus,  
                (1, 1) => temp_bool = keep_checking_x_plus_y_plus,  
                (-1, 1) => temp_bool = keep_checking_x_minus_y_plus,  
                _ => temp_bool = keep_checking_x_minus_y_minus,       
                };
            


            if is_within_bounds(move_to) && does_not_land_on_own(piece, move_to, board) 
            && temp_bool {
                valid_moves.push(index_to_letter(move_to));
            }

            if is_within_bounds(move_to){
                if !does_not_land_on_other(piece, move_to, board) {
                    match dir {
                        (1, -1) => keep_checking_x_plus_y_minus = false,  
                        (1, 1) => keep_checking_x_plus_y_plus = false,  
                        (-1, 1) => keep_checking_x_minus_y_plus = false,  
                        _ => keep_checking_x_minus_y_minus = false,       
                        };
                }
            }
            
        }
        
    }
    
    valid_moves
}

pub fn knight_moves(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>{
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
        if is_within_bounds(j) && does_not_land_on_own(piece, j, board) {
            valid_moves.push(index_to_letter(j));
        }
    }
    valid_moves

}

pub fn pawn_moves(piece: &str, board: &Vec<Vec<String>>) -> Vec<String>{ 
    let mut valid_moves: Vec<String> = vec![];
    let index = get_position_index(piece, board);
    let first_down = (index.0 + 2, index.1);
    let first_up = (index.0 - 2, index.1);
    let down = (index.0 + 1, index.1);
    let up = (index.0 - 1, index.1);
    let down_left = (index.0 + 1, index.1 - 1);
    let down_right = (index.0 + 1, index.1 + 1);
    let up_left = (index.0 - 1, index.1 - 1);
    let up_right = (index.0 - 1, index.1 + 1); 

    if (other_functions::get_color(piece).as_str() == "b") && get_position_index(piece, board).0 == 1{
        valid_moves.push(other_functions::index_to_letter(first_down)); 
    }
    if (other_functions::get_color(piece).as_str() == "w") && get_position_index(piece, board).0 == 6{
        valid_moves.push(other_functions::index_to_letter(first_up)); 
    }
    
    if (other_functions::get_color(piece).as_str() == "b") && is_within_bounds(down) && does_not_land_on_own(piece, down, board) {
        valid_moves.push(other_functions::index_to_letter(down)); 
        
        if is_within_bounds(down_left) && does_not_land_on_own(piece, down_left, board) &&
        other_functions::get_color(&other_functions::get_piece_at(&board, down_left.0 as usize, down_left.1 as usize)) == "w" { 
            valid_moves.push(other_functions::index_to_letter(down_left)); 
        }
        else if is_within_bounds(down_right) && does_not_land_on_own(piece, down_right, board) &&
        other_functions::get_color(&other_functions::get_piece_at(&board, down_right.0 as usize, down_right.1 as usize)) == "w"  { 
            valid_moves.push(other_functions::index_to_letter(down_right)); 
        }
        
    }
    else if is_within_bounds(up) && does_not_land_on_own(piece, up, board){
        valid_moves.push(other_functions::index_to_letter(up)); 
      
        if is_within_bounds(up_left) && does_not_land_on_own(piece, up_left, board) && 
        other_functions::get_color(&other_functions::get_piece_at(&board, up_left.0 as usize, up_left.1 as usize)) == "b" { 
            valid_moves.push(other_functions::index_to_letter(up_left)); 
        }
        else if is_within_bounds(up_right) && does_not_land_on_own(piece, up_right, board) &&
        other_functions::get_color(&other_functions::get_piece_at(&board, up_right.0 as usize, up_right.1 as usize)) == "b" { 
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

fn does_not_land_on_own(piece: &str, move_to: (i32, i32), board: &Vec<Vec<String>>) -> bool {
    &board[move_to.0 as usize][move_to.1 as usize][0..1] != other_functions::get_color(piece).as_str()
} 
fn does_not_land_on_other(piece: &str, move_to: (i32, i32), board: &Vec<Vec<String>>) -> bool {
    &board[move_to.0 as usize][move_to.1 as usize][0..1] != get_opposite_color(piece).as_str()
} 

//Om piece color = vit -> vitas drag
//Kolla vilken ruta vit kung är på
//Kolla alla svarta pjäsers valid moves
//Om vit kung ruta == valid move hos svart pjäs
//Vit är i schack
pub fn is_in_check(piece: &str, board: &Vec<Vec<String>>) -> bool{

    let mut is_in_check = false;


    let mut king_of_certain_color = "";
    let mut king_of_opposite_color = "";

    if other_functions::get_color(piece).as_str() == "w"{
        king_of_certain_color = "wKI";
        king_of_opposite_color = "bKI";
    }
    else{
        king_of_certain_color = "bKI";
        king_of_opposite_color = "wKI";
    }


    let pieces_to_try: Vec<String> 
    = get_pieces_of_certain_color(&king_of_opposite_color, board); //Maybe should be opposite

    //Does king need to be tested?
    //pieces_to_try.retain(|x| x != "wKI");
    //pieces_to_try.retain(|x| x != "bKI");



    for p in pieces_to_try {
        //println!("Piece to try for is_in_check: {p}"); //For testing
        for move_to in other_functions::valid_moves(&p, board){ //Detta blir rekursivt
            let move_to_index = letter_to_index(move_to.as_str());
            let piece_at_move_to: &String 
            = get_piece_at(board, move_to_index.0 as usize, move_to_index.1 as usize);


            if piece_at_move_to == &king_of_certain_color.to_string(){

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
fn solves_check_piece(piece: &str, board: &Vec<Vec<String>>) -> bool { //Ska bara kolla för en piece?
    let mut solves_check = false;
    let mut temp_board: Vec<Vec<String>> = board.clone();
    let index_move_from = get_position_index(piece, board);

    for mov in other_functions::valid_moves(piece, board) { 
        let index_move_to = letter_to_index(mov.as_str());

        let piece_at_move_to = 
        get_piece_at(board, index_move_to.0 as usize, index_move_to.1 as usize);


        //Move piece to move_to
        temp_board[index_move_from.0 as usize][index_move_from.1 as usize] = "   ".to_string();
        temp_board[index_move_to.0 as usize][index_move_to.1 as usize] = piece.to_string();

        //Check if move solves check
        if !is_in_check(&piece, &temp_board) { 
            solves_check = true;
        }

        //Reverse move 
        temp_board[index_move_from.0 as usize][index_move_from.1 as usize] = piece.to_string();
        temp_board[index_move_to.0 as usize][index_move_to.1 as usize] = piece_at_move_to.to_string();
    }
    

    solves_check
}

pub fn solves_check_all_pieces(piece: String, board: &Vec<Vec<String>>) -> Vec<String>{
    let mut pieces_that_solve_check: Vec<String> = Vec::new();
    let mut pieces_that_have_been_checked: Vec<String>  = Vec::new(); //Necesary or not?
    let pieces_to_try: Vec<String> = get_pieces_of_certain_color(&piece, board);

    for p in pieces_to_try.iter(){
        if solves_check_piece(p, board) {
            pieces_that_solve_check.push(p.to_string());
        }
        pieces_that_have_been_checked.push(p.to_string()); //Necesary or not?

    }
    


    pieces_that_solve_check
}

pub fn solves_check_move(piece: &str, board: &Vec<Vec<String>>, mov: String) -> bool{

    let mut solves_check = false;
    let mut temp_board: Vec<Vec<String>> = board.clone();

    let index_move_from = get_position_index(piece, board);
    let index_move_to = letter_to_index(mov.as_str());

    let piece_at_move_to = 
        get_piece_at(board, index_move_to.0 as usize, index_move_to.1 as usize);


    //Move piece to move_to
    temp_board[index_move_from.0 as usize][index_move_from.1 as usize] = "   ".to_string();
    temp_board[index_move_to.0 as usize][index_move_to.1 as usize] = piece.to_string();

    //Check if move solves check
    if !is_in_check(&piece, &temp_board) { 
        solves_check = true;
    }

    //Reverse move  -> probably not neccesary
    temp_board[index_move_from.0 as usize][index_move_from.1 as usize] = piece.to_string();
    temp_board[index_move_to.0 as usize][index_move_to.1 as usize] = piece_at_move_to.to_string();

    if solves_check {
        println!("This move solves check: {mov}"); //For testing
    }
    

    solves_check
}


fn get_pieces_of_certain_color(piece: &str, board: &Vec<Vec<String>>) -> Vec<String> {
    let pieces_to_check: Vec<String> = board.iter().flat_map(|row| row.iter())
    .filter(|&x| x[0..1] == other_functions::get_color(piece)).map(|y| y.to_string()).collect();
    pieces_to_check
}

fn get_opposite_color(piece: &str) -> String{
    let mut opposite_color = String::new();
    if other_functions::get_color(piece) == "w"{
        opposite_color.push_str("b");
    }
    else if other_functions::get_color(piece) == "b" {
        opposite_color.push_str("w");
    }
    opposite_color

}
