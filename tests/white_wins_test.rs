use checkers_rs::state::{GameSession, State, StateTransition};
use checkers_rs::state::states::{GameOverState};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::board::Board;

#[test]
fn test_white_wins_display() {
    // Test 1: White wins by eliminating all black pieces
    let session = GameSession::new();
    let state = GameOverState::new(Some(Color::White));
    let view_data = state.get_view_data(&session);
    
    println!("Test 1 - White wins by elimination:");
    println!("Winner: {:?}", Some(Color::White));
    println!("Message: {}", view_data.status_message);
    
    assert_eq!(view_data.status_message, "White wins!");
}

#[test]
fn test_black_wins_display() {
    // Test 2: Black wins
    let session = GameSession::new();
    let state = GameOverState::new(Some(Color::Black));
    let view_data = state.get_view_data(&session);
    
    println!("Test 2 - Black wins:");
    println!("Winner: {:?}", Some(Color::Black));
    println!("Message: {}", view_data.status_message);
    
    assert_eq!(view_data.status_message, "Black wins!");
}

#[test]
fn test_white_wins_by_black_stalemate() {
    // Test 3: White wins because black has no moves
    let mut session = GameSession::new();
    session.game.board = Board::new(8);
    
    // White pieces
    session.game.board.set_piece(3, 3, Some(Piece::new(Color::White)));
    session.game.board.set_piece(4, 4, Some(Piece::new(Color::White)));
    
    // Black has no pieces, so is in stalemate
    session.game.current_player = Color::Black;
    
    // When Black is in stalemate, White wins
    let state = GameOverState::new(Some(Color::White));
    let view_data = state.get_view_data(&session);
    
    println!("Test 3 - White wins by black stalemate:");
    println!("Current player: {:?}", session.game.current_player);
    println!("Winner: {:?}", Some(Color::White));
    println!("Message: {}", view_data.status_message);
    
    assert_eq!(view_data.status_message, "White wins!");
}

#[test]
fn test_check_winner_logic() {
    // Test the actual check_winner function
    let mut session = GameSession::new();
    session.game.board = Board::new(8);
    
    // Only white pieces remain
    session.game.board.set_piece(3, 3, Some(Piece::new(Color::White)));
    session.game.board.set_piece(4, 4, Some(Piece::new(Color::White)));
    
    let winner = session.game.check_winner();
    println!("Test 4 - check_winner with only white pieces:");
    println!("Winner from check_winner: {:?}", winner);
    
    assert_eq!(winner, Some(Color::White));
    
    // Now test GameOverState with this winner
    let state = GameOverState::new(winner);
    let view_data = state.get_view_data(&session);
    
    println!("Message from GameOverState: {}", view_data.status_message);
    assert_eq!(view_data.status_message, "White wins!");
}