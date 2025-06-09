use checkers_rs::core::game::{CheckersGame, GameError};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::Position;

#[test]
fn test_make_move_coords_returns_new_game() {
    let game1 = CheckersGame::new();

    let from = Position::new(5, 0);
    let to = Position::new(4, 1);

    let initial_board = game1.board.clone();
    let initial_player = game1.current_player;

    let (game2, continue_capture) = game1
        .make_move_coords(from.row, from.col, to.row, to.col)
        .unwrap();

    assert!(game2.board.get_piece(to.row, to.col).is_some());
    assert!(game2.board.get_piece(from.row, from.col).is_none());
    assert_eq!(game2.current_player, Color::Black);
    assert!(!continue_capture);

    assert_eq!(game1.board.cells, initial_board.cells);
    assert_eq!(game1.current_player, initial_player);
}

#[test]
fn test_make_move_handles_forced_capture() {
    let mut game1 = CheckersGame::new();

    game1.board.cells = vec![vec![None; 8]; 8];
    
    game1.board.cells[4][3] = Some(Piece::new(Color::White));
    game1.board.cells[3][4] = Some(Piece::new(Color::Black));
    game1.board.cells[2][5] = None;
    
    let from = Position::new(4, 3);
    let to = Position::new(3, 2);

    let result = game1.make_move_coords(from.row, from.col, to.row, to.col);

    assert!(matches!(result, Err(GameError::ForcedCaptureAvailable)));
}

#[test]
fn test_make_move_handles_king_promotion() {
    let mut game1 = CheckersGame::new();

    game1.board.cells = vec![vec![None; 8]; 8];
    game1.board.cells[1][0] = Some(Piece::new(Color::White));

    let from = Position::new(1, 0);
    let to = Position::new(0, 1);

    let (game2, _) = game1
        .make_move_coords(from.row, from.col, to.row, to.col)
        .unwrap();

    let piece_in_game2 = game2.board.get_piece(to.row, to.col).unwrap();
    assert!(piece_in_game2.is_king);

    let piece_in_game1 = game1.board.get_piece(from.row, from.col).unwrap();
    assert!(!piece_in_game1.is_king);
}

#[test]
fn test_with_switched_player_returns_new_game() {
    let game1 = CheckersGame::new();
    assert_eq!(game1.current_player, Color::White);

    let game2 = game1.with_switched_player();

    assert_eq!(game2.current_player, Color::Black);
    assert_eq!(game1.current_player, Color::White);
}

#[test]
fn test_make_move_capture_immutability() {
    let mut game1 = CheckersGame::new();

    game1.board.cells[4][1] = Some(Piece::new(Color::Black));
    game1.board.cells[3][2] = None;
    game1.board.cells[2][3] = Some(Piece::new(Color::Black));
    game1.board.cells[1][4] = None;

    let from = Position::new(5, 0);
    let to = Position::new(3, 2);

    let initial_board = game1.board.clone();

    let (game2, continue_capture) = game1
        .make_move_coords(from.row, from.col, to.row, to.col)
        .unwrap();

    assert!(game2.board.get_piece(to.row, to.col).is_some());
    assert!(game2.board.get_piece(from.row, from.col).is_none());
    assert!(game2.board.get_piece(4, 1).is_none());
    assert!(continue_capture);

    assert_eq!(game1.board.cells, initial_board.cells);
    assert!(game1.board.get_piece(4, 1).is_some());
}

#[test]
fn test_check_winner_immutability() {
    let mut game1 = CheckersGame::new();

    game1.board.cells = vec![vec![None; 8]; 8];
    let mut king_piece = Piece::new(Color::White);
    king_piece.promote_to_king();
    game1.board.cells[0][0] = Some(king_piece);

    let winner = game1.check_winner();
    assert_eq!(winner, Some(Color::White));

    assert!(game1.board.get_piece(0, 0).is_some());
}
