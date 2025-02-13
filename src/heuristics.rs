use std::path::absolute;

use crate::board::*;

/// A heuristic function to estimate the cost of reaching the goal state from a given board.
///
/// ```rust
/// let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
/// let h = Heuristic::Manhattan.estimate(&board);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heuristic {
    /// The blind heuristic always returns 0.
    Blind,
    /// The Hamming heuristic, which counts the number of misplaced tiles.
    Hamming,
    /// The Manhattan heuristic, which computes the sum of the Manhattan distances of each tile to its goal position.
    Manhattan,
}

impl Heuristic {
    pub fn estimate(&self, board: &Board) -> u32 {
        match self {
            // blind heuristic always returns 0
            Heuristic::Blind => 0,
            Heuristic::Hamming => {
                let mut amount = 0;
                for i in 0..3 {
                    for j in 0..3 {
                        if Board::GOAL.value_at(i, j) != board.value_at(i, j) {
                            amount += 1;
                        }
                    }
                }

                return amount;
            }
            Heuristic::Manhattan => {
                let mut distance = 0;

                for n in 0..9 {
                    let (x1, y1) = board.position(n);
                    let (x2, y2) = Board::GOAL.position(n);

                    distance += x1.abs_diff(x2) + y1.abs_diff(y2);
                }

                return distance as u32;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_heuristic() {
        use super::*;
        let board = Board::new([[8, 7, 3], [2, 0, 5], [1, 4, 6]]);
        assert_eq!(Heuristic::Blind.estimate(&board), 0);
        assert_eq!(Heuristic::Hamming.estimate(&board), 8);
        assert_eq!(Heuristic::Manhattan.estimate(&board), 16);
    }
}
