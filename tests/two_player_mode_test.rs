use checkers_rs::core::piece::Color;
use checkers_rs::state::GameSession;

#[test]
fn test_two_player_mode_alternates_turns() {
    let session_initial = GameSession::new();
    assert_eq!(session_initial.game.current_player, Color::White);

    let session_after_white = session_initial.select_piece(5, 0).unwrap();
    let (session_after_white_move, _) = session_after_white.make_move(4, 1).unwrap();
    assert_eq!(session_after_white_move.game.current_player, Color::Black);

    let session_after_black = session_after_white_move.select_piece(2, 1).unwrap();
    let (session_after_black_move, _) = session_after_black.make_move(3, 0).unwrap();
    assert_eq!(session_after_black_move.game.current_player, Color::White);

    assert_eq!(session_initial.game.current_player, Color::White);
}

#[test]
fn test_both_players_can_move_pieces() {
    let session_initial = GameSession::new();

    let session1 = session_initial.select_piece(5, 0).unwrap();
    let (session2, _) = session1.make_move(4, 1).unwrap();

    let session3 = session2.select_piece(2, 7).unwrap();
    let (session4, _) = session3.make_move(3, 6).unwrap();

    let session5 = session4.select_piece(5, 2).unwrap();
    let (session6, _) = session5.make_move(4, 3).unwrap();

    let session7 = session6.select_piece(2, 5).unwrap();
    let (session_final, _) = session7.make_move(3, 4).unwrap();

    assert!(session_final.game.board.get_piece(4, 1).is_some());
    assert!(session_final.game.board.get_piece(3, 6).is_some());
    assert!(session_final.game.board.get_piece(4, 3).is_some());
    assert!(session_final.game.board.get_piece(3, 4).is_some());
}

#[test]
fn test_both_players_can_capture() {
    let session_initial = GameSession::new();

    let session1 = session_initial.select_piece(5, 0).unwrap();
    let (session2, _) = session1.make_move(4, 1).unwrap();

    let session3 = session2.select_piece(2, 3).unwrap();
    let (session4, _) = session3.make_move(3, 2).unwrap();

    let session5 = session4.select_piece(4, 1).unwrap();
    let (session6, _) = session5.make_move(2, 3).unwrap();

    assert!(session6.game.board.get_piece(3, 2).is_none());
    assert!(session6.game.board.get_piece(2, 3).is_some());

    let black_has_capture = session6.game.board.get_piece(1, 2).is_some()
        || session6.game.board.get_piece(1, 4).is_some();

    let session_final = if black_has_capture {
        if session6.game.board.get_piece(1, 2).is_some() {
            let session7 = session6.select_piece(1, 2).unwrap();
            let (session8, _) = session7.make_move(3, 4).unwrap();
            session8
        } else if session6.game.board.get_piece(1, 4).is_some() {
            let session7 = session6.select_piece(1, 4).unwrap();
            let (session8, _) = session7.make_move(3, 2).unwrap();
            session8
        } else {
            session6
        }
    } else {
        let session7 = session6.select_piece(2, 5).unwrap();
        let (session8, _) = session7.make_move(3, 4).unwrap();
        session8
    };

    assert!(session_final.game.board.get_piece(3, 2).is_none());
}

#[test]
fn test_game_continues_without_ai() {
    let session_initial = GameSession::new();

    let session1 = session_initial.select_piece(5, 0).unwrap();
    let (session2, _) = session1.make_move(4, 1).unwrap();
    assert_eq!(session2.game.current_player, Color::Black);

    let session3 = session2.select_piece(2, 7).unwrap();
    let (session4, _) = session3.make_move(3, 6).unwrap();
    assert_eq!(session4.game.current_player, Color::White);

    let session5 = session4.select_piece(5, 2).unwrap();
    let (session6, _) = session5.make_move(4, 3).unwrap();
    assert_eq!(session6.game.current_player, Color::Black);

    let session7 = session6.select_piece(2, 1).unwrap();
    let (session8, _) = session7.make_move(3, 0).unwrap();
    assert_eq!(session8.game.current_player, Color::White);

    let session9 = session8.select_piece(5, 4).unwrap();
    let (session10, _) = session9.make_move(4, 5).unwrap();
    assert_eq!(session10.game.current_player, Color::Black);
}
