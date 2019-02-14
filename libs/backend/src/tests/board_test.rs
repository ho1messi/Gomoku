use super::super::board::*;
use super::super::board::MoveDirection::*;
use super::super::cross_point::*;

#[test]
fn index_valid() {
    let b = Board::new();

    for index in 0..300 {
        if index < b.cp_count() {
            assert_eq!(b.is_index_valid(index), true);
        } else {
            assert_eq!(b.is_index_valid(index), false);
        }
    }
}

#[test]
fn chess() {
    for size in 1..20 {
        let mut b = Board::create_by_size(size);

        for index in 0..b.cp_count() {
            assert_eq!(b.have_chess_at(index), false);
            assert_eq!(b.get_chess_at(index), None);

            b.clear_at(index);
            assert_eq!(b.have_chess_at(index), false);
            assert_eq!(b.get_chess_at(index), None);

            b.put_chess_at(index, ChessType::CtBlack);
            assert_eq!(b.have_chess_at(index), true);
            assert_eq!(b.get_chess_at(index), Some(ChessType::CtBlack));

            b.remove_chess_at(index);
            assert_eq!(b.have_chess_at(index), false);
            assert_eq!(b.get_chess_at(index), None);

            b.put_chess_at(index, ChessType::CtWhite);
            assert_eq!(b.have_chess_at(index), true);
            assert_eq!(b.get_chess_at(index), Some(ChessType::CtWhite));

            b.override_chess_at(index, ChessType::CtBlack);
            assert_eq!(b.have_chess_at(index), true);
            assert_eq!(b.get_chess_at(index), Some(ChessType::CtBlack));

            b.override_chess_at(index, ChessType::CtWhite);
            assert_eq!(b.have_chess_at(index), true);
            assert_eq!(b.get_chess_at(index), Some(ChessType::CtWhite));

            b.clear_at(index);
            assert_eq!(b.have_chess_at(index), false);
            assert_eq!(b.get_chess_at(index), None);
        }
    }
}

#[test]
fn move_valid() {
    let b = Board::new();

    for row in 1..14 {
        for col in 1..14 {
            let index = Coord{row, col}.to_index(b.size());

            assert_eq!(b.move_by_direction(index, MdUp).unwrap(),
                       Coord{row: row - 1, col}.to_index(b.size()));
            assert_eq!(b.move_by_direction(index, MdDown).unwrap(),
                       Coord{row: row + 1, col}.to_index(b.size()));
            assert_eq!(b.move_by_direction(index, MdLeft).unwrap(),
                       Coord{row, col: col - 1}.to_index(b.size()));
            assert_eq!(b.move_by_direction(index, MdRight).unwrap(),
                       Coord{row, col: col + 1}.to_index(b.size()));
            assert_eq!(b.move_by_direction(index, MdUpLeft).unwrap(),
                       Coord{row: row - 1, col: col - 1}.to_index(b.size()));
            assert_eq!(b.move_by_direction(index, MdUpRight).unwrap(),
                       Coord{row: row - 1, col: col + 1}.to_index(b.size()));
            assert_eq!(b.move_by_direction(index, MdDownLeft).unwrap(),
                       Coord{row: row + 1, col: col - 1}.to_index(b.size()));
            assert_eq!(b.move_by_direction(index, MdDownRight).unwrap(),
                       Coord{row: row + 1, col: col + 1}.to_index(b.size()));
        }
    }
}

#[test]
fn move_invalid() {
    let b = Board::new();

    for i in 0..15 {
        let index = Coord{row: 0, col: i}.to_index(b.size());
        assert_eq!(b.move_by_direction(index, MdUp).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdUpLeft).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdUpRight).is_err(), true);

        let index = Coord{row: 14, col: i}.to_index(b.size());
        assert_eq!(b.move_by_direction(index, MdDown).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdDownLeft).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdDownRight).is_err(), true);

        let index = Coord{row: i, col: 0}.to_index(b.size());
        assert_eq!(b.move_by_direction(index, MdLeft).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdUpLeft).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdDownLeft).is_err(), true);

        let index = Coord{row: i, col: 14}.to_index(b.size());
        assert_eq!(b.move_by_direction(index, MdRight).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdUpRight).is_err(), true);
        assert_eq!(b.move_by_direction(index, MdDownRight).is_err(), true);
    }
}
