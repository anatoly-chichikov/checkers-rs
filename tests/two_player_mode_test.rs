use checkers_rs::core::board::Grid;
use checkers_rs::core::game::Game;
use checkers_rs::core::piece::Color;
use checkers_rs::core::Position;
use checkers_rs::state::GameSession;

#[test]
fn test_two_player_mode_alternates_turns() {
    let session = GameSession::new();
    let base = session.game.turn();
    let session = session.select_piece(5, 0).unwrap();
    let (session, _) = session.make_move(4, 1).unwrap();
    let turn = session.game.turn();
    let session = session.select_piece(2, 1).unwrap();
    let (session, _) = session.make_move(3, 0).unwrap();
    assert!(
        base == Color::White && turn == Color::Black && session.game.turn() == Color::White,
        "turns did not alternate"
    );
}

#[test]
fn test_both_players_can_move_pieces() {
    let session = GameSession::new();
    let session = session.select_piece(5, 0).unwrap();
    let (session, _) = session.make_move(4, 1).unwrap();
    let session = session.select_piece(2, 7).unwrap();
    let (session, _) = session.make_move(3, 6).unwrap();
    let session = session.select_piece(5, 2).unwrap();
    let (session, _) = session.make_move(4, 3).unwrap();
    let session = session.select_piece(2, 5).unwrap();
    let (session, _) = session.make_move(3, 4).unwrap();
    let board = session.game.board();
    assert!(
        board.piece(Position { row: 4, col: 1 }).is_some()
            && board.piece(Position { row: 3, col: 6 }).is_some()
            && board.piece(Position { row: 4, col: 3 }).is_some()
            && board.piece(Position { row: 3, col: 4 }).is_some(),
        "pieces did not remain after moves"
    );
}

#[test]
fn test_both_players_can_capture() {
    let session = GameSession::new();
    let session = session.select_piece(5, 0).unwrap();
    let (session, _) = session.make_move(4, 1).unwrap();
    let session = session.select_piece(2, 3).unwrap();
    let (session, _) = session.make_move(3, 2).unwrap();
    let session = session.select_piece(4, 1).unwrap();
    let (session, _) = session.make_move(2, 3).unwrap();
    let flag;
    let west;
    let east;
    {
        let board = session.game.board();
        flag = board.piece(Position { row: 3, col: 2 }).is_none()
            && board.piece(Position { row: 2, col: 3 }).is_some();
        west = board.piece(Position { row: 1, col: 2 }).is_some();
        east = board.piece(Position { row: 1, col: 4 }).is_some();
    }
    let session = if west {
        let session = session.select_piece(1, 2).unwrap();
        let (session, _) = session.make_move(3, 4).unwrap();
        session
    } else if east {
        let session = session.select_piece(1, 4).unwrap();
        let (session, _) = session.make_move(3, 2).unwrap();
        session
    } else {
        let session = session.select_piece(2, 5).unwrap();
        let (session, _) = session.make_move(3, 4).unwrap();
        session
    };
    let board = session.game.board();
    assert!(
        flag && board.piece(Position { row: 3, col: 2 }).is_none(),
        "capture sequence did not remove expected pieces"
    );
}

#[test]
fn test_game_continues_without_ai() {
    let session = GameSession::new();
    let session = session.select_piece(5, 0).unwrap();
    let (session, _) = session.make_move(4, 1).unwrap();
    let mut turns = Vec::new();
    turns.push(session.game.turn());
    let session = session.select_piece(2, 7).unwrap();
    let (session, _) = session.make_move(3, 6).unwrap();
    turns.push(session.game.turn());
    let session = session.select_piece(5, 2).unwrap();
    let (session, _) = session.make_move(4, 3).unwrap();
    turns.push(session.game.turn());
    let session = session.select_piece(2, 1).unwrap();
    let (session, _) = session.make_move(3, 0).unwrap();
    turns.push(session.game.turn());
    let session = session.select_piece(5, 4).unwrap();
    let (session, _) = session.make_move(4, 5).unwrap();
    turns.push(session.game.turn());
    assert!(
        turns
            == vec![
                Color::Black,
                Color::White,
                Color::Black,
                Color::White,
                Color::Black
            ],
        "turn sequence did not match"
    );
}
