use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::game::{Game, GameSeed, Seed as GameSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::Position;
use checkers_rs::state::states::PieceSelectedState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_piece_selected_state_makes_a_valid_move() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 5, col: 2 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 0, col: 3 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 0, col: 5 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    session.ui_state.cursor_pos = (4, 1);
    session.ui_state.selected_piece = Some((5, 2));
    session.ui_state.possible_moves = vec![(4, 1), (4, 3)];
    let state = PieceSelectedState::new((5, 2));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::Playing;
    }
    flag = flag
        && result
            .game
            .board()
            .piece(Position { row: 5, col: 2 })
            .is_none()
        && result
            .game
            .board()
            .piece(Position { row: 4, col: 1 })
            .is_some()
        && result.game.turn() == Color::Black
        && session
            .game
            .board()
            .piece(Position { row: 5, col: 2 })
            .is_some()
        && session
            .game
            .board()
            .piece(Position { row: 4, col: 1 })
            .is_none();
    assert!(flag, "valid move did not apply or transition");
}

#[test]
fn test_piece_selected_state_deselects_piece() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (2, 1);
    session.ui_state.selected_piece = Some((2, 1));
    let state = PieceSelectedState::new((2, 1));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::Playing;
    }
    flag = flag
        && result.ui_state.selected_piece.is_none()
        && session.ui_state.selected_piece == Some((2, 1));
    assert!(flag, "piece deselection did not occur");
}

#[test]
fn test_piece_selected_state_transitions_to_multi_capture() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 5, col: 0 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 4, col: 1 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 2, col: 3 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    session.ui_state.cursor_pos = (3, 2);
    session.ui_state.selected_piece = Some((5, 0));
    session.ui_state.possible_moves = vec![(3, 2)];
    let state = PieceSelectedState::new((5, 0));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::MultiCapture;
    }
    flag = flag
        && result
            .game
            .board()
            .piece(Position { row: 3, col: 2 })
            .is_some()
        && result
            .game
            .board()
            .piece(Position { row: 4, col: 1 })
            .is_none()
        && result.ui_state.selected_piece.is_some()
        && session
            .game
            .board()
            .piece(Position { row: 5, col: 0 })
            .is_some()
        && session
            .game
            .board()
            .piece(Position { row: 4, col: 1 })
            .is_some();
    assert!(flag, "multi capture transition did not occur");
}

#[test]
fn test_piece_selected_state_cursor_movement() {
    let session = GameSession::new();
    let state = PieceSelectedState::new((2, 1));
    let start = session.ui_state.cursor_pos;
    let rise = (start.0.saturating_sub(1), start.1);
    let shift = (rise.0, (rise.1 + 1).min(7));
    let (session, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Up));
    let (session, phase) = state.handle_input(&session, KeyEvent::from(KeyCode::Right));
    let flag = step == StateTransition::None
        && phase == StateTransition::None
        && session.ui_state.cursor_pos == shift
        && rise == (start.0.saturating_sub(1), start.1);
    assert!(flag, "cursor movement did not update correctly");
}

#[test]
fn test_piece_selected_state_exits_on_esc() {
    let session = GameSession::new();
    let state = PieceSelectedState::new((2, 1));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Esc));
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::Playing;
    }
    flag = flag && result.ui_state.selected_piece.is_none();
    assert!(flag, "esc did not exit selection state");
}

#[test]
fn test_piece_selected_state_rejects_invalid_move() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (5, 5);
    let state = PieceSelectedState::new((2, 1));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    let flag = step == StateTransition::None
        && result.ui_state.cursor_pos == session.ui_state.cursor_pos
        && same;
    assert!(flag, "invalid move was not rejected");
}

#[test]
fn test_piece_selected_state_view_data() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 2, col: 1 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    session.ui_state.cursor_pos = (3, 0);
    session.ui_state.selected_piece = Some((2, 1));
    let state = PieceSelectedState::new((2, 1));
    let view = state.get_view_data(&session);
    let flag = view.selected_piece == Some((2, 1))
        && view.cursor_pos == (3, 0)
        && view.status_message == "Select a square to move to"
        && !view.show_ai_thinking;
    assert!(flag, "view data did not reflect selection");
}

#[test]
fn test_piece_selected_state_game_over_transition() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 2, col: 1 },
        Some(Piece {
            color: Color::White,
            king: true,
        }),
    );
    board.place(
        Position { row: 3, col: 2 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    session.ui_state.cursor_pos = (4, 3);
    session.ui_state.selected_piece = Some((2, 1));
    session.ui_state.possible_moves = vec![(1, 0), (1, 2), (3, 0), (3, 2), (4, 3)];
    let state = PieceSelectedState::new((2, 1));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let winner = result.game.winner();
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = if winner.is_some() {
            state.state_type() == checkers_rs::state::StateType::GameOver
        } else {
            state.state_type() == checkers_rs::state::StateType::Playing
        };
    }
    flag = flag
        && result
            .game
            .board()
            .piece(Position { row: 3, col: 2 })
            .is_none();
    assert!(flag, "game over transition did not match winner");
}
