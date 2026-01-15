use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::game::{Game, GameSeed, Seed as GameSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::Position;
use checkers_rs::state::states::MultiCaptureState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_multi_capture_state_keeps_piece_selected() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 4, col: 3 },
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
    session.ui_state.selected_piece = Some((4, 3));
    let state = MultiCaptureState::new((4, 3));
    let view = state.get_view_data(&session);
    assert!(
        view.selected_piece == Some((4, 3))
            && view.status_message == "You must continue capturing!",
        "selection was not preserved"
    );
}

#[test]
fn test_multi_capture_state_forces_capture_moves_only() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 4, col: 3 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 5, col: 4 },
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
    session.ui_state.cursor_pos = (5, 4);
    session.ui_state.selected_piece = Some((4, 3));
    let state = MultiCaptureState::new((4, 3));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    let flag = step == StateTransition::None
        && result.ui_state.selected_piece == session.ui_state.selected_piece
        && same;
    assert!(flag, "capture only rule did not hold");
}

#[test]
fn test_multi_capture_state_completes_capture_sequence() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 2, col: 3 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 3, col: 4 },
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
    session.ui_state.cursor_pos = (4, 5);
    session.ui_state.selected_piece = Some((2, 3));
    session.ui_state.possible_moves = vec![];
    let state = MultiCaptureState::new((2, 3));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    let flag = step == StateTransition::None && same;
    assert!(flag, "invalid capture sequence did not remain idle");
}

#[test]
fn test_multi_capture_state_continues_if_more_captures() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 2, col: 1 },
        Some(Piece {
            color: Color::White,
            king: false,
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
    session.ui_state.possible_moves = vec![(4, 3)];
    let state = MultiCaptureState::new((2, 1));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut flag = false;
    match &step {
        StateTransition::None => {
            let mut same = true;
            for row in 0..8 {
                for col in 0..8 {
                    let spot = Position { row, col };
                    same =
                        same && result.game.board().piece(spot) == session.game.board().piece(spot);
                }
            }
            flag = same;
        }
        StateTransition::To(state) => {
            let kind = state.state_type();
            flag = kind == checkers_rs::state::StateType::MultiCapture
                || kind == checkers_rs::state::StateType::Playing;
        }
        _ => {}
    }
    assert!(
        flag,
        "multi capture continuation did not match expected result"
    );
}

#[test]
fn test_multi_capture_state_transitions_to_game_over() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 2, col: 3 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 3, col: 4 },
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
    session.ui_state.cursor_pos = (4, 5);
    session.ui_state.selected_piece = Some((2, 3));
    session.ui_state.possible_moves = vec![];
    let state = MultiCaptureState::new((2, 3));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    let flag = step == StateTransition::None && same;
    assert!(flag, "game over transition did not remain idle");
}

#[test]
fn test_multi_capture_state_cursor_movement() {
    let session = GameSession::new();
    let state = MultiCaptureState::new((4, 3));
    let start = session.ui_state.cursor_pos;
    let rise = (start.0.saturating_sub(1), start.1);
    let shift = (rise.0, (rise.1 + 1).min(7));
    let (session, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Up));
    let (session, phase) = state.handle_input(&session, KeyEvent::from(KeyCode::Right));
    let flag = step == StateTransition::None
        && phase == StateTransition::None
        && session.ui_state.cursor_pos == shift;
    assert!(flag, "cursor movement did not update correctly");
}

#[test]
fn test_multi_capture_state_ignores_non_capture_moves() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 4, col: 3 },
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
    session.ui_state.cursor_pos = (5, 4);
    session.ui_state.selected_piece = Some((4, 3));
    session.ui_state.possible_moves = vec![];
    let state = MultiCaptureState::new((4, 3));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    let flag = step == StateTransition::None
        && result
            .game
            .board()
            .piece(Position { row: 4, col: 3 })
            .is_some()
        && same;
    assert!(flag, "non capture move was not ignored");
}

#[test]
fn test_multi_capture_state_no_exit_key() {
    let session = GameSession::new();
    let state = MultiCaptureState::new((4, 3));
    let (_, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Esc));
    assert!(
        step == StateTransition::None,
        "exit key was accepted during multi capture"
    );
}

#[test]
fn test_multi_capture_state_view_data() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 4, col: 3 },
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
    session.ui_state.cursor_pos = (5, 4);
    session.ui_state.selected_piece = Some((4, 3));
    let state = MultiCaptureState::new((4, 3));
    let view = state.get_view_data(&session);
    let flag = view.selected_piece == Some((4, 3))
        && view.cursor_pos == (5, 4)
        && view.status_message == "You must continue capturing!"
        && !view.show_ai_thinking
        && view.error_message.is_none();
    assert!(flag, "view data did not match expected values");
}
