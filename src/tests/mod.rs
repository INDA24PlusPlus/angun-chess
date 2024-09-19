
use crate::other_functions::*;
use crate::valid_moves::*;

const BOARD_SIZE: usize = 8;

pub fn return_board() -> Vec<Vec<String>>{
        //Create pieces
    let black_pieces: Vec<&str> = vec![
        "bR1", "bK1", "bB1", "bKI", "bQU", "bB2", "bK2", "bR2", 
        "bP1", "bP2", "bP3", "bP4", "bP5", "bP6", "bP7", "bP8"
    ];

    let white_pieces: Vec<&str> = vec![
        "wR1", "wK1", "wB1", "wKI", "wQU", "wB2", "wK2", "wR2", 
        "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8"
    ];

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
    board
}


//Tests for other_functions

#[test]
pub fn test_get_piece_at(){
    let board = return_board();
    assert_eq!(get_piece_at(&board, 6 as usize, 3 as usize), &"wP4".to_string());
}

#[test]
pub fn test_get_color(){
    let board = return_board();
    assert!(get_color(get_piece_at(&board, 6 as usize, 3 as usize).as_str()) == "w");
}

#[test]
pub fn test_letter_to_index(){
    let input = "d6";
    let output_should_be = (2, 3);
    let output_should_not_be = (5,6);

    assert_ne!(letter_to_index(input), output_should_not_be);
    assert_eq!(letter_to_index(input), output_should_be);
}

//Tests for valid_moves

#[test]
pub fn test_pawn_moves(){
    let board = return_board();
    let moves_should_be = vec!["c5".to_string(), "c6".to_string()];

    assert_eq!(pawn_moves("bP3", &board), moves_should_be);
}

#[test]
pub fn test_knight_moves(){
    let board = return_board();
    let moves_should_be = vec!["a3".to_string(), "c3".to_string()];
    let moves_should_not_be = vec!["a5".to_string(), "d3".to_string()];

    assert_ne!(knight_moves("wK1", &board), moves_should_not_be);
    assert_eq!(knight_moves("wK1", &board), moves_should_be);
}

#[test]
pub fn test_get_pieces_of_certain_color(){
    let pieces_str = vec![
        "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8", 
        "wR1", "wK1", "wB1", "wKI", "wQU", "wB2", "wK2", "wR2"
    ];
    let pieces_should_be: Vec<String> = pieces_str.iter().map(|x| x.to_string())
    .collect();


    let board = return_board();

    assert_eq!(get_pieces_of_certain_color("wK1", &board), pieces_should_be);
}

