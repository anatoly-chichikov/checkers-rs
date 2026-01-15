use crate::core::piece::{Color, Piece};
use crate::core::{Move, Position};

#[derive(Clone, Debug)]
pub struct Board {
    size: usize,
    cells: Vec<Vec<Option<Piece>>>,
}

pub struct BoardSeed {
    pub size: usize,
}

pub trait Seed {
    /// Creates an empty board of the configured size
    fn make(self) -> Board;
}

pub trait Grid {
    /// Seeds the board with the standard starting layout
    fn seed(&mut self);
    /// Returns the piece at the supplied position
    fn piece(&self, spot: Position) -> Option<Piece>;
    /// Places a piece at the supplied position
    fn place(&mut self, spot: Position, piece: Option<Piece>) -> bool;
    /// Moves a piece from one position to another
    fn shift(&mut self, origin: Position, target: Position) -> bool;
    /// Returns true when the position is within board bounds
    fn bounds(&self, spot: Position) -> bool;
    /// Returns the board edge length
    fn edge(&self) -> usize;
    /// Returns possible destination positions for a piece
    fn moves(&self, spot: Position) -> Vec<Position>;
    /// Returns a capture path between two positions when available
    fn path(&self, origin: Position, target: Position) -> Option<Vec<Position>>;
    /// Returns all valid moves for a player
    fn choices(&self, player: Color) -> Vec<Move>;
    /// Returns true when the move is valid
    fn validity(&self, origin: Position, target: Position) -> bool;
    /// Returns true when the piece can capture
    fn capture(&self, spot: Position) -> bool;
    /// Returns true when the piece can continue capturing
    fn chain(&self, spot: Position) -> bool;
    /// Returns true when any piece can capture
    fn captures(&self, player: Color) -> bool;
    /// Returns true when the player has no legal moves
    fn stalemate(&self, player: Color) -> bool;
    /// Returns positions of pieces that can capture
    fn takers(&self, player: Color) -> Vec<Position>;
    /// Returns the winning color when only one remains
    fn winner(&self) -> Option<Color>;
}

impl Seed for BoardSeed {
    fn make(self) -> Board {
        let cells = vec![vec![None; self.size]; self.size];
        Board {
            size: self.size,
            cells,
        }
    }
}

impl Board {
    /// Walks capture chains and collects capture paths
    fn trace(
        &self,
        spot: Position,
        piece: Piece,
        path: Vec<Position>,
        paths: &mut Vec<Vec<Position>>,
    ) {
        let mut chain = false;
        let steps = if piece.king {
            vec![(-2, -2), (-2, 2), (2, -2), (2, 2)]
        } else {
            match piece.color {
                Color::White => vec![(-2, -2), (-2, 2)],
                Color::Black => vec![(2, -2), (2, 2)],
            }
        };
        for (rise, run) in steps {
            let row = spot.row as i32 + rise;
            let col = spot.col as i32 + run;
            if row < 0 || row >= self.size as i32 || col < 0 || col >= self.size as i32 {
                continue;
            }
            let mid = Position {
                row: ((spot.row as i32 + row) / 2) as usize,
                col: ((spot.col as i32 + col) / 2) as usize,
            };
            let target = Position {
                row: row as usize,
                col: col as usize,
            };
            if self.cells[target.row][target.col].is_none() {
                if let Some(rival) = self.cells[mid.row][mid.col] {
                    if rival.color != piece.color {
                        let permit = if piece.king {
                            true
                        } else {
                            match piece.color {
                                Color::White => row < spot.row as i32,
                                Color::Black => row > spot.row as i32,
                            }
                        };
                        if permit {
                            let mut board = self.clone();
                            board.cells[mid.row][mid.col] = None;
                            let mut path = path.clone();
                            path.push(target);
                            chain = true;
                            board.trace(target, piece, path, paths);
                        }
                    }
                }
            }
        }
        if !chain && !path.is_empty() {
            paths.push(path);
        }
    }
}

impl Grid for Board {
    fn seed(&mut self) {
        for row in 0..self.size {
            for col in 0..self.size {
                self.cells[row][col] = None;
            }
        }
        for row in 0..3 {
            if row % 2 == 0 {
                for col in (1..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece {
                        color: Color::Black,
                        king: false,
                    });
                }
            } else {
                for col in (0..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece {
                        color: Color::Black,
                        king: false,
                    });
                }
            }
        }
        for row in (self.size - 3)..self.size {
            if row % 2 == 0 {
                for col in (1..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece {
                        color: Color::White,
                        king: false,
                    });
                }
            } else {
                for col in (0..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece {
                        color: Color::White,
                        king: false,
                    });
                }
            }
        }
    }
    fn piece(&self, spot: Position) -> Option<Piece> {
        if self.bounds(spot) {
            self.cells[spot.row][spot.col]
        } else {
            None
        }
    }
    fn place(&mut self, spot: Position, piece: Option<Piece>) -> bool {
        if self.bounds(spot) {
            self.cells[spot.row][spot.col] = piece;
            true
        } else {
            false
        }
    }
    fn shift(&mut self, origin: Position, target: Position) -> bool {
        if !self.bounds(origin) || !self.bounds(target) {
            return false;
        }
        let piece = self.piece(origin);
        if piece.is_none() {
            return false;
        }
        self.place(target, piece);
        self.place(origin, None);
        true
    }
    fn bounds(&self, spot: Position) -> bool {
        spot.row < self.size && spot.col < self.size
    }
    fn edge(&self) -> usize {
        self.size
    }
    fn moves(&self, spot: Position) -> Vec<Position> {
        let piece = match self.piece(spot) {
            Some(piece) => piece,
            None => return vec![],
        };
        let mut ends: Vec<Position> = Vec::new();
        let mut paths: Vec<Vec<Position>> = Vec::new();
        self.trace(spot, piece, Vec::new(), &mut paths);
        if !paths.is_empty() {
            for path in paths {
                if let Some(tail) = path.last() {
                    if !ends.contains(tail) {
                        ends.push(*tail);
                    }
                }
            }
            return ends;
        }
        let mut moves: Vec<Position> = Vec::new();
        let steps = if piece.king {
            vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]
        } else {
            match piece.color {
                Color::White => vec![(-1, -1), (-1, 1)],
                Color::Black => vec![(1, -1), (1, 1)],
            }
        };
        for (rise, run) in steps {
            let row = spot.row as i32 + rise;
            let col = spot.col as i32 + run;
            if row < 0 || row >= self.size as i32 || col < 0 || col >= self.size as i32 {
                continue;
            }
            let target = Position {
                row: row as usize,
                col: col as usize,
            };
            if self.validity(spot, target) {
                moves.push(target);
            }
        }
        moves
    }
    fn path(&self, origin: Position, target: Position) -> Option<Vec<Position>> {
        let piece = self.piece(origin)?;
        let mut paths = Vec::new();
        self.trace(origin, piece, Vec::new(), &mut paths);
        for path in paths {
            if let Some(&tail) = path.last() {
                if tail == target {
                    return Some(path);
                }
            }
        }
        None
    }
    fn choices(&self, player: Color) -> Vec<Move> {
        let mut captures: Vec<Move> = Vec::new();
        let mut moves: Vec<Move> = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(piece) = self.cells[row][col] {
                    if piece.color == player {
                        let spot = Position { row, col };
                        let targets = self.moves(spot);
                        for target in targets {
                            let rise = target.row as i32 - row as i32;
                            let run = target.col as i32 - col as i32;
                            let jump = rise.abs() == 2;
                            let span = run.abs() == 2;
                            if jump && span {
                                captures.push(Move {
                                    origin: spot,
                                    target,
                                });
                            } else if rise.abs() == 1 && run.abs() == 1 {
                                moves.push(Move {
                                    origin: spot,
                                    target,
                                });
                            }
                        }
                    }
                }
            }
        }
        if !captures.is_empty() {
            captures
        } else {
            moves
        }
    }
    fn validity(&self, origin: Position, target: Position) -> bool {
        if !self.bounds(target) {
            return false;
        }
        if self.piece(target).is_some() {
            return false;
        }
        let piece = match self.piece(origin) {
            Some(piece) => piece,
            None => return false,
        };
        let rise = target.row as i32 - origin.row as i32;
        let run = target.col as i32 - origin.col as i32;
        if run.abs() != rise.abs() {
            return false;
        }
        if piece.king {
            if rise.abs() == 2 {
                let mid = Position {
                    row: ((origin.row as i32 + target.row as i32) / 2) as usize,
                    col: ((origin.col as i32 + target.col as i32) / 2) as usize,
                };
                if let Some(rival) = self.piece(mid) {
                    if rival.color != piece.color {
                        return true;
                    }
                }
                return false;
            }
            if rise.abs() == 1 {
                return true;
            }
            return false;
        } else {
            if rise.abs() == 1 {
                return match piece.color {
                    Color::White => rise < 0,
                    Color::Black => rise > 0,
                };
            }
            if rise.abs() == 2 {
                let mid = Position {
                    row: ((origin.row as i32 + target.row as i32) / 2) as usize,
                    col: ((origin.col as i32 + target.col as i32) / 2) as usize,
                };
                if let Some(rival) = self.piece(mid) {
                    if rival.color != piece.color {
                        return match piece.color {
                            Color::White => rise < 0,
                            Color::Black => rise > 0,
                        };
                    }
                }
                return false;
            }
        }
        false
    }
    fn capture(&self, spot: Position) -> bool {
        if let Some(piece) = self.piece(spot) {
            let steps = if piece.king {
                vec![(-2, -2), (-2, 2), (2, -2), (2, 2)]
            } else {
                match piece.color {
                    Color::White => vec![(-2, -2), (-2, 2)],
                    Color::Black => vec![(2, -2), (2, 2)],
                }
            };
            for (rise, run) in steps {
                let row = spot.row as i32 + rise;
                let col = spot.col as i32 + run;
                if row < 0 || row >= self.size as i32 || col < 0 || col >= self.size as i32 {
                    continue;
                }
                let mid = ((spot.row as i32 + row) / 2, (spot.col as i32 + col) / 2);
                if mid.0 < 0 || mid.0 >= self.size as i32 || mid.1 < 0 || mid.1 >= self.size as i32
                {
                    continue;
                }
                if self.cells[row as usize][col as usize].is_none() {
                    if let Some(rival) = self.cells[mid.0 as usize][mid.1 as usize] {
                        if rival.color != piece.color {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    fn chain(&self, spot: Position) -> bool {
        if let Some(piece) = self.piece(spot) {
            let steps = if piece.king {
                vec![(2, 2), (2, -2), (-2, 2), (-2, -2)]
            } else {
                match piece.color {
                    Color::White => vec![(-2, 2), (-2, -2)],
                    Color::Black => vec![(2, 2), (2, -2)],
                }
            };
            for (rise, run) in steps {
                let row = match (spot.row as i32 + rise).try_into() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let col = match (spot.col as i32 + run).try_into() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
                let target = Position { row, col };
                if self.bounds(target) && self.validity(spot, target) {
                    return true;
                }
            }
        }
        false
    }
    fn captures(&self, player: Color) -> bool {
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(piece) = self.cells[row][col] {
                    if piece.color == player {
                        let steps = if piece.king {
                            vec![(2, 2), (2, -2), (-2, 2), (-2, -2)]
                        } else {
                            match piece.color {
                                Color::White => vec![(-2, 2), (-2, -2)],
                                Color::Black => vec![(2, 2), (2, -2)],
                            }
                        };
                        for (rise, run) in steps {
                            let rank = match (row as i32 + rise).try_into() {
                                Ok(val) => val,
                                Err(_) => continue,
                            };
                            let file = match (col as i32 + run).try_into() {
                                Ok(val) => val,
                                Err(_) => continue,
                            };
                            let target = Position {
                                row: rank,
                                col: file,
                            };
                            if self.bounds(target) && self.validity(Position { row, col }, target) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
    fn stalemate(&self, player: Color) -> bool {
        if self.captures(player) {
            return false;
        }
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(piece) = self.cells[row][col] {
                    if piece.color == player {
                        let steps = if piece.king {
                            vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
                        } else {
                            match piece.color {
                                Color::White => vec![(-1, 1), (-1, -1)],
                                Color::Black => vec![(1, 1), (1, -1)],
                            }
                        };
                        for (rise, run) in steps {
                            let rank = match (row as i32 + rise).try_into() {
                                Ok(val) => val,
                                Err(_) => continue,
                            };
                            let file = match (col as i32 + run).try_into() {
                                Ok(val) => val,
                                Err(_) => continue,
                            };
                            let target = Position {
                                row: rank,
                                col: file,
                            };
                            if self.bounds(target) && self.validity(Position { row, col }, target) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
    fn takers(&self, player: Color) -> Vec<Position> {
        let mut spots = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(piece) = self.cells[row][col] {
                    if piece.color == player && self.capture(Position { row, col }) {
                        spots.push(Position { row, col });
                    }
                }
            }
        }
        spots
    }
    fn winner(&self) -> Option<Color> {
        let mut white = false;
        let mut black = false;
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(piece) = self.cells[row][col] {
                    match piece.color {
                        Color::White => white = true,
                        Color::Black => black = true,
                    }
                    if white && black {
                        return None;
                    }
                }
            }
        }
        match (white, black) {
            (true, false) => Some(Color::White),
            (false, true) => Some(Color::Black),
            _ => None,
        }
    }
}
