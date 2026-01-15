use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::game::{Game, GameSeed, Seed as GameSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::{Move, Position};
#[test]
fn test_should_promote() {
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 1, col: 0 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    let game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let (white, _) = game
        .play(Move {
            origin: Position { row: 1, col: 0 },
            target: Position { row: 0, col: 1 },
        })
        .unwrap();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 6, col: 1 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    let game = GameSeed {
        board,
        turn: Color::Black,
    }
    .make();
    let (black, _) = game
        .play(Move {
            origin: Position { row: 6, col: 1 },
            target: Position { row: 7, col: 0 },
        })
        .unwrap();
    assert!(
        matches!(
            white.board().piece(Position { row: 0, col: 1 }),
            Some(Piece { king: true, .. })
        ) && matches!(
            black.board().piece(Position { row: 7, col: 0 }),
            Some(Piece { king: true, .. })
        )
    );
}
#[test]
fn test_is_valid_move() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            board.place(Position { row: row, col: col }, None);
        }
    }
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    assert!(board.validity(Position { row: 5, col: 2 }, Position { row: 4, col: 3 }));
    assert!(board.validity(Position { row: 5, col: 2 }, Position { row: 4, col: 1 }));
    assert!(!board.validity(Position { row: 5, col: 2 }, Position { row: 6, col: 3 }));
    assert!(!board.validity(Position { row: 5, col: 2 }, Position { row: 6, col: 1 }));
    board.place(Position { row: 2, col: 3 }, Some(black_piece));
    assert!(board.validity(Position { row: 2, col: 3 }, Position { row: 3, col: 4 }));
    assert!(board.validity(Position { row: 2, col: 3 }, Position { row: 3, col: 2 }));
    assert!(!board.validity(Position { row: 2, col: 3 }, Position { row: 1, col: 4 }));
    assert!(!board.validity(Position { row: 2, col: 3 }, Position { row: 1, col: 2 }));
}
#[test]
fn test_capture_move() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            board.place(Position { row: row, col: col }, None);
        }
    }
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    board.place(Position { row: 4, col: 3 }, Some(black_piece));
    board.place(Position { row: 3, col: 4 }, None);
    assert!(board.validity(Position { row: 5, col: 2 }, Position { row: 3, col: 4 }));
    board.place(Position { row: 2, col: 5 }, Some(black_piece));
    board.place(Position { row: 3, col: 4 }, Some(white_piece));
    board.place(Position { row: 4, col: 3 }, None);
    assert!(board.validity(Position { row: 2, col: 5 }, Position { row: 4, col: 3 }));
}
#[test]
fn test_king_movement() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_king = Piece {
        color: Color::White,
        king: true,
    };
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            board.place(Position { row: row, col: col }, None);
        }
    }
    board.place(Position { row: 4, col: 4 }, Some(white_king));
    assert!(board.validity(Position { row: 4, col: 4 }, Position { row: 3, col: 3 }));
    assert!(board.validity(Position { row: 4, col: 4 }, Position { row: 3, col: 5 }));
    assert!(board.validity(Position { row: 4, col: 4 }, Position { row: 5, col: 3 }));
    assert!(board.validity(Position { row: 4, col: 4 }, Position { row: 5, col: 5 }));
}
#[test]
fn test_has_more_captures() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            board.place(Position { row: row, col: col }, None);
        }
    }
    board.place(Position { row: 4, col: 3 }, Some(white_piece));
    board.place(Position { row: 3, col: 2 }, Some(black_piece));
    assert!(board.chain(Position { row: 4, col: 3 }));
}
#[test]
fn test_has_captures_available() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            board.place(Position { row: row, col: col }, None);
        }
    }
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    board.place(Position { row: 4, col: 3 }, Some(black_piece));
    board.place(Position { row: 3, col: 4 }, None);
    assert!(board.captures(Color::White));
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            if let Some(piece) = board.piece(Position { row: row, col: col }) {
                if piece.color == Color::Black {
                    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    for (dr, dc) in directions {
                        let r = (row as i32 + dr) as usize;
                        let c = (col as i32 + dc) as usize;
                        if board.bounds(Position { row: r, col: c }) {
                            if let Some(p) = board.piece(Position { row: r, col: c }) {
                                if p.color == Color::White {
                                    let r2 = (r as i32 + dr) as usize;
                                    let c2 = (c as i32 + dc) as usize;
                                    if board.bounds(Position { row: r2, col: c2 }) {
                                        board.place(
                                            Position { row: r2, col: c2 },
                                            Some(white_piece),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(!board.captures(Color::Black));
}
#[test]
fn test_is_stalemate() {
    let mut board = BoardSeed { size: 8 }.make();
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            board.place(Position { row: row, col: col }, None);
        }
    }
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 0, col: 1 }, Some(black_piece));
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    board.place(Position { row: 1, col: 0 }, Some(white_piece));
    board.place(Position { row: 0, col: 1 }, Some(black_piece));
    assert!(board.stalemate(Color::White));
    assert!(!board.stalemate(Color::Black));
}
#[test]
fn test_check_winner() {
    let mut board = BoardSeed { size: 8 }.make();
    for row in 0..board.edge() {
        for col in 0..board.edge() {
            board.place(Position { row: row, col: col }, None);
        }
    }
    assert_eq!(board.winner(), None);
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    assert_eq!(board.winner(), Some(Color::White));
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 2, col: 3 }, Some(black_piece));
    assert_eq!(board.winner(), None);
    board.place(Position { row: 5, col: 2 }, None);
    assert_eq!(board.winner(), Some(Color::Black));
}
#[test]
fn test_can_piece_capture_positive_white_regular() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    board.place(Position { row: 4, col: 3 }, Some(black_piece));
    assert!(board.capture(Position { row: 5, col: 2 }));
}
#[test]
fn test_can_piece_capture_positive_black_regular() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 2, col: 2 }, Some(black_piece));
    board.place(Position { row: 3, col: 3 }, Some(white_piece));
    assert!(board.capture(Position { row: 2, col: 2 }));
}
#[test]
fn test_can_piece_capture_positive_white_king() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_king = Piece {
        color: Color::White,
        king: true,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 5, col: 2 }, Some(white_king));
    board.place(Position { row: 4, col: 3 }, Some(black_piece));
    assert!(board.capture(Position { row: 5, col: 2 }));
    board.place(Position { row: 4, col: 3 }, None);
    board.place(Position { row: 6, col: 3 }, Some(black_piece));
    assert!(board.capture(Position { row: 5, col: 2 }));
}
#[test]
fn test_can_piece_capture_positive_black_king() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_king = Piece {
        color: Color::Black,
        king: true,
    };
    board.place(Position { row: 2, col: 2 }, Some(black_king));
    board.place(Position { row: 3, col: 3 }, Some(white_piece));
    assert!(board.capture(Position { row: 2, col: 2 }));
    board.place(Position { row: 3, col: 3 }, None);
    board.place(Position { row: 1, col: 3 }, Some(white_piece));
    assert!(board.capture(Position { row: 2, col: 2 }));
}
#[test]
fn test_can_piece_capture_negative_no_opponent_piece() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    assert!(!board.capture(Position { row: 5, col: 2 }));
}
#[test]
fn test_can_piece_capture_negative_landing_blocked() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    board.place(Position { row: 4, col: 3 }, Some(black_piece));
    board.place(Position { row: 3, col: 4 }, Some(white_piece));
    assert!(!board.capture(Position { row: 5, col: 2 }));
}
#[test]
fn test_can_piece_capture_negative_landing_out_of_bounds() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 1, col: 0 }, Some(white_piece));
    board.place(Position { row: 0, col: 1 }, Some(black_piece));
    assert!(!board.capture(Position { row: 1, col: 0 }));
}
#[test]
fn test_can_piece_capture_negative_opponent_but_no_empty_landing() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 5, col: 2 }, Some(white_piece));
    board.place(Position { row: 4, col: 3 }, Some(black_piece));
    board.place(Position { row: 3, col: 4 }, Some(black_piece));
    assert!(!board.capture(Position { row: 5, col: 2 }));
}
#[test]
fn test_can_piece_capture_negative_wrong_direction_regular_white() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 3, col: 2 }, Some(white_piece));
    board.place(Position { row: 4, col: 3 }, Some(black_piece));
    assert!(!board.capture(Position { row: 3, col: 2 }));
}
#[test]
fn test_can_piece_capture_negative_wrong_direction_regular_black() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    let black_piece = Piece {
        color: Color::Black,
        king: false,
    };
    board.place(Position { row: 5, col: 2 }, Some(black_piece));
    board.place(Position { row: 4, col: 3 }, Some(white_piece));
    assert!(!board.capture(Position { row: 5, col: 2 }));
}
#[test]
fn test_can_piece_capture_negative_no_piece_at_coords() {
    let board = BoardSeed { size: 8 }.make();
    assert!(!board.capture(Position { row: 5, col: 2 }));
}
#[test]
fn test_can_piece_capture_negative_piece_no_moves() {
    let mut board = BoardSeed { size: 8 }.make();
    let white_piece = Piece {
        color: Color::White,
        king: false,
    };
    board.place(Position { row: 0, col: 0 }, Some(white_piece));
    board.place(
        Position { row: 1, col: 1 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    assert!(!board.capture(Position { row: 0, col: 0 }));
    let mut board2 = BoardSeed { size: 8 }.make();
    let white_piece2 = Piece {
        color: Color::White,
        king: false,
    };
    board2.place(Position { row: 7, col: 0 }, Some(white_piece2));
    board2.place(
        Position { row: 6, col: 1 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    assert!(!board2.capture(Position { row: 7, col: 0 }));
}
#[cfg(test)]
mod get_all_possible_moves_tests {
    use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
    use checkers_rs::core::piece::{Color, Piece};
    use checkers_rs::core::Position;
    use std::collections::HashSet;
    fn assert_moves_equal(actual: &[Position], expected: &[(usize, usize)]) {
        let actual_set: HashSet<_> = actual.iter().map(|pos| (pos.row, pos.col)).collect();
        let expected_set: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(
            actual_set, expected_set,
            "Actual moves: {:?}, Expected moves: {:?}",
            actual, expected
        );
    }
    #[test]
    fn test_regular_white_no_captures_forward() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_piece = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 5, col: 2 }, Some(white_piece));
        let moves = board.moves(Position { row: 5, col: 2 });
        let expected_moves = vec![(4, 1), (4, 3)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_black_no_captures_forward() {
        let mut board = BoardSeed { size: 8 }.make();
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 2, col: 2 }, Some(black_piece));
        let moves = board.moves(Position { row: 2, col: 2 });
        let expected_moves = vec![(3, 1), (3, 3)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_white_edge_no_captures() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_piece = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 5, col: 0 }, Some(white_piece));
        let moves = board.moves(Position { row: 5, col: 0 });
        let expected_moves = vec![(4, 1)];
        assert_moves_equal(&moves, &expected_moves);
        board.place(Position { row: 5, col: 0 }, None);
        board.place(Position { row: 5, col: 7 }, Some(white_piece));
        let moves_edge7 = board.moves(Position { row: 5, col: 7 });
        let expected_moves_edge7 = vec![(4, 6)];
        assert_moves_equal(&moves_edge7, &expected_moves_edge7);
    }
    #[test]
    fn test_regular_black_edge_no_captures() {
        let mut board = BoardSeed { size: 8 }.make();
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 2, col: 0 }, Some(black_piece));
        let moves = board.moves(Position { row: 2, col: 0 });
        let expected_moves = vec![(3, 1)];
        assert_moves_equal(&moves, &expected_moves);
        board.place(Position { row: 2, col: 0 }, None);
        board.place(Position { row: 2, col: 7 }, Some(black_piece));
        let moves_edge7 = board.moves(Position { row: 2, col: 7 });
        let expected_moves_edge7 = vec![(3, 6)];
        assert_moves_equal(&moves_edge7, &expected_moves_edge7);
    }
    #[test]
    fn test_regular_white_blocked_no_captures() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_piece = Piece {
            color: Color::White,
            king: false,
        };
        let friendly_blocking_piece = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 5, col: 2 }, Some(white_piece));
        board.place(Position { row: 4, col: 1 }, Some(friendly_blocking_piece));
        board.place(Position { row: 4, col: 3 }, Some(friendly_blocking_piece));
        let moves = board.moves(Position { row: 5, col: 2 });
        let expected_moves: Vec<(usize, usize)> = vec![];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_black_blocked_no_captures() {
        let mut board = BoardSeed { size: 8 }.make();
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        let friendly_blocking_piece = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 2, col: 2 }, Some(black_piece));
        board.place(Position { row: 3, col: 1 }, Some(friendly_blocking_piece));
        board.place(Position { row: 3, col: 3 }, Some(friendly_blocking_piece));
        let moves = board.moves(Position { row: 2, col: 2 });
        let expected_moves: Vec<(usize, usize)> = vec![];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_king_no_captures_all_directions() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_king = Piece {
            color: Color::White,
            king: true,
        };
        board.place(Position { row: 3, col: 3 }, Some(white_king));
        let moves = board.moves(Position { row: 3, col: 3 });
        let expected_moves = vec![(2, 2), (2, 4), (4, 2), (4, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_king_edge_no_captures() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_king = Piece {
            color: Color::White,
            king: true,
        };
        board.place(Position { row: 0, col: 3 }, Some(white_king));
        let moves = board.moves(Position { row: 0, col: 3 });
        let expected_moves = vec![(1, 2), (1, 4)];
        assert_moves_equal(&moves, &expected_moves);
        board.place(Position { row: 0, col: 3 }, None);
        board.place(Position { row: 3, col: 0 }, Some(white_king));
        let moves_left_edge = board.moves(Position { row: 3, col: 0 });
        let expected_moves_left_edge = vec![(2, 1), (4, 1)];
        assert_moves_equal(&moves_left_edge, &expected_moves_left_edge);
    }
    #[test]
    fn test_king_blocked_no_captures() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_king = Piece {
            color: Color::White,
            king: true,
        };
        let friendly_blocking_piece = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 3, col: 3 }, Some(white_king));
        board.place(
            Position { row: 2, col: 2 },
            Some(friendly_blocking_piece.clone()),
        );
        board.place(
            Position { row: 2, col: 4 },
            Some(friendly_blocking_piece.clone()),
        );
        board.place(
            Position { row: 4, col: 2 },
            Some(friendly_blocking_piece.clone()),
        );
        board.place(
            Position { row: 4, col: 4 },
            Some(friendly_blocking_piece.clone()),
        );
        let moves = board.moves(Position { row: 3, col: 3 });
        let expected_moves: Vec<(usize, usize)> = vec![];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_white_single_capture() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_piece = Piece {
            color: Color::White,
            king: false,
        };
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 5, col: 2 }, Some(white_piece));
        board.place(Position { row: 4, col: 3 }, Some(black_piece));
        let moves = board.moves(Position { row: 5, col: 2 });
        let expected_moves = vec![(3, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_white_multiple_single_capture_options() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_piece = Piece {
            color: Color::White,
            king: false,
        };
        let black_piece1 = Piece {
            color: Color::Black,
            king: false,
        };
        let black_piece2 = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 5, col: 2 }, Some(white_piece));
        board.place(Position { row: 4, col: 1 }, Some(black_piece1));
        board.place(Position { row: 4, col: 3 }, Some(black_piece2));
        let moves = board.moves(Position { row: 5, col: 2 });
        let expected_moves = vec![(3, 0), (3, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_black_single_capture() {
        let mut board = BoardSeed { size: 8 }.make();
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        let white_piece = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 2, col: 2 }, Some(black_piece));
        board.place(Position { row: 3, col: 3 }, Some(white_piece));
        let moves = board.moves(Position { row: 2, col: 2 });
        let expected_moves = vec![(4, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_black_multiple_single_capture_options() {
        let mut board = BoardSeed { size: 8 }.make();
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        let white_piece1 = Piece {
            color: Color::White,
            king: false,
        };
        let white_piece2 = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 2, col: 2 }, Some(black_piece));
        board.place(Position { row: 3, col: 1 }, Some(white_piece1));
        board.place(Position { row: 3, col: 3 }, Some(white_piece2));
        let moves = board.moves(Position { row: 2, col: 2 });
        let expected_moves = vec![(4, 0), (4, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_king_single_capture_all_directions() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_king = Piece {
            color: Color::White,
            king: true,
        };
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 3, col: 3 }, Some(white_king));
        board.place(Position { row: 2, col: 2 }, Some(black_piece.clone()));
        board.place(Position { row: 2, col: 4 }, Some(black_piece.clone()));
        board.place(Position { row: 4, col: 2 }, Some(black_piece.clone()));
        board.place(Position { row: 4, col: 4 }, Some(black_piece.clone()));
        let moves = board.moves(Position { row: 3, col: 3 });
        let expected_moves = vec![(1, 1), (1, 5), (5, 1), (5, 5)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_king_single_capture_prefers_capture_over_regular() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_king = Piece {
            color: Color::White,
            king: true,
        };
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 3, col: 3 }, Some(white_king));
        board.place(Position { row: 2, col: 2 }, Some(black_piece.clone()));
        let moves = board.moves(Position { row: 3, col: 3 });
        let expected_moves = vec![(1, 1)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_white_double_capture() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_piece = Piece {
            color: Color::White,
            king: false,
        };
        let black_piece1 = Piece {
            color: Color::Black,
            king: false,
        };
        let black_piece2 = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 7, col: 0 }, Some(white_piece));
        board.place(Position { row: 6, col: 1 }, Some(black_piece1));
        board.place(Position { row: 4, col: 3 }, Some(black_piece2));
        let moves = board.moves(Position { row: 7, col: 0 });
        let expected_moves = vec![(3, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_black_double_capture() {
        let mut board = BoardSeed { size: 8 }.make();
        let black_piece = Piece {
            color: Color::Black,
            king: false,
        };
        let white_piece1 = Piece {
            color: Color::White,
            king: false,
        };
        let white_piece2 = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 0, col: 1 }, Some(black_piece));
        board.place(Position { row: 1, col: 2 }, Some(white_piece1));
        board.place(Position { row: 3, col: 4 }, Some(white_piece2));
        let moves = board.moves(Position { row: 0, col: 1 });
        let expected_moves = vec![(4, 5)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_white_multiple_multi_capture_paths() {
        let mut board = BoardSeed { size: 8 }.make();
        board.place(
            Position { row: 6, col: 1 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        board.place(
            Position { row: 5, col: 2 },
            Some(Piece {
                color: Color::Black,
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
        board.place(
            Position { row: 6, col: 5 },
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
        board.place(
            Position { row: 3, col: 2 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 5, col: 6 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 3, col: 6 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        let moves = board.moves(Position { row: 6, col: 5 });
        let expected_moves = vec![(2, 1), (2, 5)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_king_multi_capture_changing_directions() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_king = Piece {
            color: Color::White,
            king: true,
        };
        let black_piece1 = Piece {
            color: Color::Black,
            king: false,
        };
        let black_piece2 = Piece {
            color: Color::Black,
            king: false,
        };
        board.place(Position { row: 4, col: 3 }, Some(white_king));
        board.place(Position { row: 3, col: 2 }, Some(black_piece1));
        board.place(Position { row: 1, col: 2 }, Some(black_piece2));
        let moves = board.moves(Position { row: 4, col: 3 });
        let expected_moves = vec![(0, 3)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_king_multiple_multi_capture_paths() {
        let mut board = BoardSeed { size: 8 }.make();
        let king = Piece {
            color: Color::White,
            king: true,
        };
        board.place(Position { row: 3, col: 3 }, Some(king));
        board.place(
            Position { row: 2, col: 2 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 2, col: 4 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 4, col: 2 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 4, col: 4 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board = BoardSeed { size: 8 }.make();
        let k = Piece {
            color: Color::White,
            king: true,
        };
        board.place(Position { row: 3, col: 3 }, Some(k));
        board.place(
            Position { row: 2, col: 2 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 0, col: 0 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 2, col: 4 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 0, col: 6 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        let king_piece = Piece {
            color: Color::White,
            king: true,
        };
        board.place(Position { row: 4, col: 3 }, Some(king_piece));
        board.place(
            Position { row: 3, col: 2 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        board.place(
            Position { row: 1, col: 2 },
            Some(Piece {
                color: Color::Black,
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
        board.place(
            Position { row: 1, col: 6 },
            Some(Piece {
                color: Color::Black,
                king: false,
            }),
        );
        let moves = board.moves(Position { row: 4, col: 3 });
        let expected_moves = vec![(0, 3), (0, 7)];
        assert_moves_equal(&moves, &expected_moves);
    }
    #[test]
    fn test_regular_piece_no_moves_possible() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_piece1 = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 7, col: 0 }, Some(white_piece1));
        board.place(
            Position { row: 6, col: 1 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        let moves1 = board.moves(Position { row: 7, col: 0 });
        assert_moves_equal(&moves1, &[]);
        let white_piece2 = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 0, col: 1 }, Some(white_piece2));
        let moves2 = board.moves(Position { row: 0, col: 1 });
        assert_moves_equal(&moves2, &[]);
        let white_piece3 = Piece {
            color: Color::White,
            king: false,
        };
        board.place(Position { row: 5, col: 2 }, Some(white_piece3));
        board.place(
            Position { row: 4, col: 1 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        board.place(
            Position { row: 4, col: 3 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        let moves3 = board.moves(Position { row: 5, col: 2 });
        assert_moves_equal(&moves3, &[]);
    }
    #[test]
    fn test_king_no_moves_possible() {
        let mut board = BoardSeed { size: 8 }.make();
        let white_king = Piece {
            color: Color::White,
            king: true,
        };
        board.place(Position { row: 0, col: 0 }, Some(white_king));
        board.place(
            Position { row: 1, col: 1 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        let moves1 = board.moves(Position { row: 0, col: 0 });
        assert_moves_equal(&moves1, &[]);
        board.place(Position { row: 0, col: 0 }, None);
        board.place(Position { row: 1, col: 1 }, None);
        board.place(Position { row: 3, col: 3 }, Some(white_king));
        board.place(
            Position { row: 2, col: 2 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        board.place(
            Position { row: 2, col: 4 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        board.place(
            Position { row: 4, col: 2 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        board.place(
            Position { row: 4, col: 4 },
            Some(Piece {
                color: Color::White,
                king: false,
            }),
        );
        let moves_surrounded = board.moves(Position { row: 3, col: 3 });
        assert_moves_equal(&moves_surrounded, &[]);
    }
}
