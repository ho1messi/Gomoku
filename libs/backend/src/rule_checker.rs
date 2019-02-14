use std::collections::HashMap;

use super::cross_point::*;
use super::board::*;

pub struct RuleChecker {
    board: Board,
    tuples: Vec<Vec<usize>>,
    tuple_indices: HashMap<MoveDirection, usize>,
}

impl RuleChecker {
    pub fn new() -> Self {
        let mut rule_checker = RuleChecker {
            board: Board::new(),
            tuples: Vec::new(),
            tuple_indices: HashMap::new(),
        };

        rule_checker.set_all_tuples();
        return rule_checker;
    }

    pub fn create_with_board(board: &Board) -> Self {
        let mut rule_checker = RuleChecker {
            board: board.clone(),
            tuples: Vec::new(),
            tuple_indices: HashMap::new(),
        };

        rule_checker.set_all_tuples();
        return rule_checker;
    }

    pub fn get_winner(&self) -> Option<ChessType> {
        for tuple in self.tuples.iter() {
            let mut black_count = 0;
            let mut white_count = 0;

            for index in tuple.iter() {
                match self.board.get_chess_at(*index) {
                    Some(ChessType::CtBlack) => black_count += 1,
                    Some(ChessType::CtWhite) => white_count += 1,
                    None => {},
                }
            }

            if black_count == 5 {
                return Some(ChessType::CtBlack);
            } else if white_count == 5 {
                return Some(ChessType::CtWhite);
            }
        }

        return None;
    }

    fn set_all_tuples(&mut self) {
        let cp_one_line = self.board.size();
        let tp_one_line = cp_one_line - 4;

        self.tuple_indices.insert(MoveDirection::MdRight, self.tuples.len());
        self.set_tuples(0, 0, cp_one_line, tp_one_line, MoveDirection::MdRight);

        self.tuple_indices.insert(MoveDirection::MdDown, self.tuples.len());
        self.set_tuples(0, 0, tp_one_line, cp_one_line, MoveDirection::MdDown);

        self.tuple_indices.insert(MoveDirection::MdDownRight, self.tuples.len());
        self.set_tuples(0, 0, tp_one_line, tp_one_line, MoveDirection::MdDownRight);

        self.tuple_indices.insert(MoveDirection::MdDownLeft, self.tuples.len());
        self.set_tuples(0, 4, tp_one_line, tp_one_line, MoveDirection::MdDownLeft);
    }

    fn set_tuples(&mut self, row_offset: usize, col_offset: usize,
                  row_count: usize, col_count: usize, md: MoveDirection) {
        let row_end = row_offset + row_count;
        let col_end = col_offset + col_count;

        for row in row_offset..row_end {
            for col in col_offset..col_end {
                self.set_tuple(Coord{row, col}.to_index(self.board.size()), md);
            }
        }
    }

    fn set_tuple(&mut self, index: usize, md: MoveDirection) {
        let mut tuple: Vec<usize> = Vec::new();
        let mut index_t = index;

        tuple.push(index_t);
        for _ in 0..4 {
            let index_t = self.board.move_by_direction(index_t, md).unwrap();
            tuple.push(index_t);
        }

        self.tuples.push(tuple);
    }
}