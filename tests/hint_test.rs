use checkers_rs::core::move_history::MoveHistory;
use checkers_rs::core::piece::Color as PieceColor;

#[test]
fn test_move_history_creation() {
    let mut history = MoveHistory::new();
    assert_eq!(history.get_all_moves().len(), 0);

    history.add_move((0, 0), (1, 1), PieceColor::White, vec![], false);
    assert_eq!(history.get_all_moves().len(), 1);

    let last_move = history.get_last_move().unwrap();
    assert_eq!(last_move.from, (0, 0));
    assert_eq!(last_move.to, (1, 1));
    assert_eq!(last_move.player, PieceColor::White);
    assert_eq!(last_move.captured.len(), 0);
    assert_eq!(last_move.became_king, false);
}

#[test]
fn test_move_history_notation() {
    let mut history = MoveHistory::new();

    // Add a simple move
    history.add_move((5, 2), (4, 3), PieceColor::White, vec![], false);
    assert_eq!(history.to_notation(), "1. c3-d4");

    // Add a capture move
    history.add_move((2, 5), (4, 3), PieceColor::Black, vec![(3, 4)], false);
    assert_eq!(history.to_notation(), "1. c3-d4 2. f6xd4");

    // Add a move that promotes to king
    history.add_move((1, 0), (0, 1), PieceColor::White, vec![], true);
    assert_eq!(history.to_notation(), "1. c3-d4 2. f6xd4 3. a7-b8K");
}

#[test]
fn test_move_history_clear() {
    let mut history = MoveHistory::new();

    history.add_move((0, 0), (1, 1), PieceColor::White, vec![], false);
    history.add_move((2, 2), (3, 3), PieceColor::Black, vec![], false);
    assert_eq!(history.get_all_moves().len(), 2);

    history.clear();
    assert_eq!(history.get_all_moves().len(), 0);
    assert_eq!(history.get_last_move(), None);
}
