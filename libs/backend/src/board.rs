use std::collections::HashMap;

use super::cross_point::*;
use super::utils::*;

use self::MoveDirection::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum MoveDirection {
    MdUp,
    MdDown,
    MdLeft,
    MdRight,
    MdUpLeft,
    MdUpRight,
    MdDownLeft,
    MdDownRight,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn from_index(index: usize, size: usize) -> Coord {
        let row = index / size;
        let col = index % size;
        return Coord {row, col};
    }

    pub fn to_index(&self, size: usize) -> usize {
        return self.row * size + self.col;
    }
}

pub struct Step {
    pub coord: Coord,
    pub chess: ChessType,
}

#[derive(Clone)]
pub struct NeighborIndexMap {
    map: HashMap<MoveDirection, usize>,
}

impl NeighborIndexMap {
    pub fn new() -> NeighborIndexMap {
        return NeighborIndexMap {
            map: HashMap::new(),
        };
    }

    pub fn add_neighbor(&mut self, index: usize, md: MoveDirection) {
        self.map.insert(md, index);
    }

    pub fn get_neighbor(&self, md: MoveDirection) -> Option<usize> {
        match self.map.get(&md) {
            Some(index_ref) => return Some(*index_ref),
            None => return None,
        }
    }
}

#[derive(Clone)]
pub struct Board {
    size: usize,
    cp_count: usize,
    cross_points: Vec<CrossPoint>,
    neighbor_maps: Vec<NeighborIndexMap>,
}

impl Board {
    pub fn new() -> Board {
        return Board::create_by_size(15);
    }

    pub fn create_by_size(size: usize) -> Board {
        let mut b = Board {
            size,
            cp_count: size * size,
            cross_points: Vec::new(),
            neighbor_maps: Vec::new(),
        };

        b.initial_cps();
        return b;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn cp_count(&self) -> usize {
        return self.cp_count;
    }

    pub fn is_index_valid(&self, index: usize) -> bool {
        return index < self.cp_count;
    }

    pub fn have_chess_at(&self, index: usize) -> bool {
        self.check_index_panic(index);

        return self.cross_points[index].have_chess();
    }

    pub fn get_chess_at(&self, index: usize) -> Option<ChessType> {
        self.check_index_panic(index);

        return self.cross_points[index].get_chess();
    }

    pub fn put_chess_at(&mut self, index: usize, chess: ChessType) {
        self.check_index_panic(index);

        self.cross_points[index].put_chess(chess);
    }

    pub fn remove_chess_at(&mut self, index: usize) {
        self.check_index_panic(index);

        self.cross_points[index].remove_chess();
    }

    pub fn override_chess_at(&mut self, index: usize, chess: ChessType) {
        self.check_index_panic(index);

        self.cross_points[index].override_with(chess);
    }

    pub fn clear_at(&mut self, index: usize) {
        self.check_index_panic(index);

        self.cross_points[index].clear();
    }

    pub fn move_by_direction(&self, index: usize, md: MoveDirection) -> Result<usize, Error> {
        return self.move_by_map(index, md);
    }

    fn initial_cps(&mut self) {
        for _ in 0..self.cp_count {
            self.cross_points.push(CrossPoint::new());
            self.neighbor_maps.push(NeighborIndexMap::new());
        }

        let mds = vec![MdUp, MdDown, MdLeft, MdRight, MdUpLeft, MdUpRight, MdDownLeft, MdDownRight];
        for index in 0..self.cp_count {
            for md in mds.iter() {
                match self.move_by_index(index, *md) {
                    Ok(neighbor_index) => {
                        self.neighbor_maps[index].add_neighbor(neighbor_index, *md);
                    }
                    Err(_) => {},
                }
            }
        }
    }

    fn move_by_index(&self, index: usize, md: MoveDirection) -> Result<usize, Error> {
        self.check_index_panic(index);

        let mut row_i: i32 = (index / self.size) as i32;
        let mut col_i: i32 = (index % self.size) as i32;
        match md {
            MdUp => row_i = row_i - 1,
            MdDown => row_i = row_i + 1,
            MdLeft => col_i = col_i - 1,
            MdRight => col_i = col_i + 1,
            MdUpLeft => {row_i = row_i - 1; col_i = col_i - 1},
            MdUpRight => {row_i = row_i - 1; col_i = col_i + 1},
            MdDownLeft => {row_i = row_i + 1; col_i = col_i - 1},
            MdDownRight => {row_i = row_i + 1; col_i = col_i + 1},
        }

        if row_i > -1 && row_i < self.size as i32 && col_i > -1 && col_i < self.size as i32 {
            return Result::Ok(
                Coord {row: row_i as usize, col: col_i as usize}
                    .to_index(self.size)
            );
        }

        return Result::Err(
            Error::create(ErrorKind::IndexInvalid, "move index out of range")
        );
    }

    fn move_by_map(&self, index: usize, md: MoveDirection) -> Result<usize, Error> {
        match self.neighbor_maps[index].get_neighbor(md) {
            Some(neighbor_index) => return Ok(neighbor_index),
            None => return Err(Error::create(
                ErrorKind::IndexInvalid,
                "move index out of range",
            )),
        }
    }

    fn check_index_panic(&self, index: usize) {
        if !self.is_index_valid(index) {
            panic!("cross point index out of range");
        }
    }
}