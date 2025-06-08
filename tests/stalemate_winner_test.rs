use checkers_rs::state::{GameSession, State, StateTransition};
use checkers_rs::state::states::{PlayingState, PieceSelectedState, GameOverState};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::board::Board;
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_stalemate_declares_correct_winner() {
    let mut session = GameSession::new();
    
    // Clear the board
    session.game.board = Board::new(8);
    
    // Set up a scenario where white can make a move that puts black in stalemate
    // White piece that can move
    session.game.board.set_piece(5, 3, Some(Piece::new(Color::White)));
    
    // Black piece that will be trapped after white's move
    session.game.board.set_piece(7, 5, Some(Piece::new(Color::Black)));
    session.game.board.set_piece(7, 7, Some(Piece::new(Color::Black)));
    
    // White's turn
    session.game.current_player = Color::White;
    
    // Simulate white making a move
    session.ui_state.cursor_pos = (5, 3);
    let mut state: Box<dyn State> = Box::new(PlayingState::new());
    
    // Select white piece
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Enter));
    if let StateTransition::To(new_state) = transition {
        state = new_state;
    }
    
    // Move to position that doesn't directly trap black but after turn switch black has no moves
    session.ui_state.cursor_pos = (6, 4);
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Enter));
    
    // Check the transition - if black is in stalemate, who wins?
    match transition {
        StateTransition::To(new_state) => {
            // If it's a GameOverState, check the winner
            let view_data = new_state.get_view_data(&session);
            println!("Game over message: {}", view_data.status_message);
            println!("Current player after move: {:?}", session.game.current_player);
            
            // The winner should be White (who just moved and put Black in stalemate)
            assert!(
                view_data.status_message.contains("White wins"),
                "White should win when Black is in stalemate. Got: {}",
                view_data.status_message
            );
        }
        _ => {
            // Not game over, continue play
        }
    }
}

#[test]
fn test_direct_stalemate_check() {
    let mut session = GameSession::new();
    session.game.board = Board::new(8);
    
    // Only white pieces on board, black has no pieces
    session.game.board.set_piece(3, 3, Some(Piece::new(Color::White)));
    session.game.board.set_piece(4, 4, Some(Piece::new(Color::White)));
    
    // It's black's turn but they have no pieces
    session.game.current_player = Color::Black;
    
    // Check if black is in stalemate
    assert!(session.game.is_stalemate(), "Black should be in stalemate with no pieces");
    
    // Create GameOverState with the opposite of current player as winner
    let game_over_state = GameOverState::new(Some(session.game.current_player.opposite()));
    let view_data = game_over_state.get_view_data(&session);
    
    println!("Current player: {:?}", session.game.current_player);
    println!("Winner passed to GameOverState: {:?}", session.game.current_player.opposite());
    println!("Game over message: {}", view_data.status_message);
    
    assert!(
        view_data.status_message.contains("White wins"),
        "When Black is in stalemate, White should win. Got: {}",
        view_data.status_message
    );
}