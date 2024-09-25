

use angun_chess::*;
use other_functions::*;

use raylib::prelude::*;
use valid_moves::solves_check_move;

const BOARD_SIZE: usize = 8;

struct ChessEngine {
    board: Vec<Vec<String>>,
    whites_turn_to_move: bool,
}

impl ChessEngine {
    fn new() -> Self {
        // Create pieces
        let black_pieces: Vec<&str> = vec![
            "bR1", "bK1", "bB1", "bQU", "bKI", "bB2", "bK2", "bR2", 
            "bP1", "bP2", "bP3", "bP4", "bP5", "bP6", "bP7", "bP8"
        ];
        
        let white_pieces: Vec<&str> = vec![
            "wR1", "wK1", "wB1", "wQU", "wKI", "wB2", "wK2", "wR2", 
            "wP1", "wP2", "wP3", "wP4", "wP5", "wP6", "wP7", "wP8"
        ];

        // Set turn order
        let whites_turn_to_move = true;

        // Initilize board
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

        Self {
            board,
            whites_turn_to_move,
        }
    }

    fn get_valid_pieces(&self) -> Vec<String> {
        //Checking which pieces can be moved
        
        let is_in_check = valid_moves::is_in_check(if self.whites_turn_to_move { "wKI" } else { "bKI" }, &self.board);

        valid_pieces(self.whites_turn_to_move, &self.board, is_in_check)
    }

    fn get_valid_moves(&self, piece_to_move: &String) -> Vec<String> {

        let avaliable_pieces = self.get_valid_pieces();
        
        let is_in_check = valid_moves::is_in_check(if self.whites_turn_to_move { "wKI" } else { "bKI" }, &self.board);
        
        if !avaliable_pieces.contains(piece_to_move) {
            return Vec::new();
        }

        //If check -> filter out positions that do not solve check
        let moves = valid_moves(piece_to_move.as_str(), &self.board);

        if is_in_check {
            moves.iter().filter(|m| solves_check_move(piece_to_move, &self.board, m.to_string())).cloned().collect::<Vec<String>>()
        } else {
            moves
        }
    }

    fn do_move(&mut self, piece: &String, to: (i32, i32)) -> bool {
        let to = index_to_letter((to.1, to.0));

        println!("{}", to);
        
        let valid_moves = self.get_valid_moves(piece);
        if !valid_moves.contains(&to) {
            return false;
        }

        //Moving piece
        let move_from = get_position_index(piece, &self.board);
        let move_to = letter_to_index(to.as_str());
        self.board[move_from.0 as usize][move_from.1 as usize] = "   ".to_string();
        self.board[move_to.0 as usize][move_to.1 as usize] = piece.clone();    
    
    
        //Change turns
        self.whites_turn_to_move = !self.whites_turn_to_move;

        true
    }

    fn get_piece(&self, (x, y): (i32, i32)) -> String {
        self.board[y as usize][x as usize].clone()
    }
}
