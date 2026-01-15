use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::game::{Game, GameSeed, Seed as GameSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::Position;
use checkers_rs::state::states::AITurnState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_ai_turn_state_shows_thinking_status() {
    let mut session = GameSession::new();
    session.game = session.game.switch();
    let state = AITurnState::new();
    let view = state.get_view_data(&session);
    assert!(
        view.show_ai_thinking && view.status_message == "AI is thinking...",
        "thinking status was not shown"
    );
}

#[test]
fn test_ai_turn_state_shows_ai_error_if_present() {
    let mut session = GameSession::new();
    session.game = session.game.switch();
    session.ai_state = session.ai_state.set_error("Test error".to_string());
    let state = AITurnState::new();
    let view = state.get_view_data(&session);
    assert!(
        view.error_message == Some("Test error"),
        "ai error was not displayed"
    );
}

#[tokio::test]
async fn test_ai_turn_state_makes_ai_move() {
    std::env::set_var("AI_TEST_MODE", "1");
    let mut session = GameSession::new();
    session.game = session.game.switch();
    let state = AITurnState::new();
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Char(' ')));
    std::env::remove_var("AI_TEST_MODE");
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::Playing;
    }
    let mut mark = false;
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = result.game.board().piece(Position { row, col }) {
                if piece.color == Color::Black && row > 2 {
                    mark = true;
                }
            }
        }
    }
    flag =
        flag && result.game.turn() == Color::White && session.game.turn() == Color::Black && mark;
    assert!(flag, "ai move did not transition or move a piece");
}

#[tokio::test]
async fn test_ai_turn_state_transitions_to_game_over_if_no_moves() {
    std::env::set_var("AI_TEST_MODE", "1");
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 7, col: 7 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::Black,
    }
    .make();
    let state = AITurnState::new();
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Char(' ')));
    std::env::remove_var("AI_TEST_MODE");
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::GameOver;
    }
    flag = flag && result.game.end() && !session.game.end();
    assert!(flag, "ai turn did not end the game with no moves");
}

#[tokio::test]
async fn test_ai_turn_state_simple_ai_makes_move_on_custom_board() {
    std::env::set_var("AI_TEST_MODE", "1");
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 5, col: 0 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 7, col: 2 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::Black,
    }
    .make();
    let state = AITurnState::new();
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Char(' ')));
    std::env::remove_var("AI_TEST_MODE");
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::Playing;
    }
    flag = flag && result.game.turn() == Color::White && session.game.turn() == Color::Black;
    assert!(flag, "ai move did not switch the turn");
}
