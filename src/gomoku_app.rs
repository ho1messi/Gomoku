use backend::cross_point::*;
use backend::board::*;
use backend::rule_checker::*;

use azul::prelude::*;
use azul::widgets::label::*;
use azul::widgets::button::*;

pub struct GomokuApp {
    cur_chess: ChessType,
    board: Board,
    rule_checker: RuleChecker,
    steps: Vec<Step>,
    winner: Option<ChessType>,
}

impl GomokuApp {
    pub fn new() -> Self {
        return Self {
            cur_chess: ChessType::CtBlack,
            board: Board::new(),
            rule_checker: RuleChecker::new(),
            steps: Vec::new(),
            winner: None,
        };
    }
}

impl Layout for GomokuApp {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        let mut board = Dom::new(NodeType::Div)
            .with_class("board")
            .with_id("board");

        board.add_child(Dom::image(info.resources.get_image("board").unwrap())
            .with_class("board_img"));
        /*
        for row in 0..self.board.size() {
            let mut board_row = Dom::new(NodeType::Div)
                .with_class("board_row")
                .with_id(format!("board_row_{}", row));

            for col in 0..self.board.size() {
                let cross_point = Dom::new(NodeType::Div)
                    .with_class("cross_point")
                    .with_id(format!("cross_point_{}", row * self.board.size() + col));

                board_row.add_child(cross_point);
            }

            board.add_child(board_row);
        }
        */

        return Dom::new(NodeType::Div)
            .with_child(board);
    }
}
