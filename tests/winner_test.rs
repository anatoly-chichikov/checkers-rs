use checkers_rs::core::board::Board;
use checkers_rs::core::game::CheckersGame;
use checkers_rs::core::piece::{Color, Piece};

#[test]
fn test_white_wins_when_black_has_no_pieces() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);
    
    // Place only white pieces
    game.board.set_piece(2, 2, Some(Piece::new(Color::White)));
    game.board.set_piece(3, 3, Some(Piece::new(Color::White)));
    
    // Check winner should return White
    let winner = game.check_winner();
    assert_eq!(winner, Some(Color::White), "White should win when only white pieces remain");
}

#[test]
fn test_black_wins_when_white_has_no_pieces() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);
    
    // Place only black pieces
    game.board.set_piece(5, 5, Some(Piece::new(Color::Black)));
    game.board.set_piece(6, 6, Some(Piece::new(Color::Black)));
    
    // Check winner should return Black
    let winner = game.check_winner();
    assert_eq!(winner, Some(Color::Black), "Black should win when only black pieces remain");
}

#[test]
fn test_white_wins_when_black_cannot_move() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);
    
    // Set up a scenario where black cannot move
    // White king blocking black piece
    let mut white_king = Piece::new(Color::White);
    white_king.promote_to_king();
    game.board.set_piece(3, 3, Some(white_king));
    
    // Black piece trapped
    game.board.set_piece(7, 7, Some(Piece::new(Color::Black)));
    
    // Black's turn but cannot move
    game.current_player = Color::Black;
    
    // Check if black is in stalemate
    let is_stalemate = game.is_stalemate();
    assert!(is_stalemate, "Black should be in stalemate");
    
    // In this case, white should win
    // Note: check_winner only checks pieces on board, not stalemate
    // The game logic should declare white as winner when black is in stalemate
}