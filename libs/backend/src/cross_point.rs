use self::ChessType::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ChessType {
    CtBlack,
    CtWhite,
}

impl ChessType {
    pub fn get_another(&self) -> ChessType {
        match *self {
            CtBlack => return CtWhite,
            CtWhite => return CtBlack,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match *self {
            CtBlack => return "black chess",
            CtWhite => return "white chess",
        }
    }
}

#[derive(Clone, Debug)]
pub struct CrossPoint {
    status: Option<ChessType>,
}

impl CrossPoint {
    pub fn new() -> CrossPoint {
        return CrossPoint{status: None};
    }

    pub fn create_by_chess(chess: ChessType) -> CrossPoint {
        return CrossPoint{status: Some(chess)};
    }

    pub fn have_chess(&self) -> bool {
        return self.status != None;
    }

    pub fn get_chess(&self) -> Option<ChessType> {
        return self.status;
    }

    pub fn put_chess(&mut self, chess: ChessType) {
        if self.have_chess() {
            panic!("can not put chess in the nonempty cross point");
        }

        self.status = Some(chess);
    }

    pub fn remove_chess(&mut self) {
        if !self.have_chess() {
            panic!("there are no chess to remove in the cross point");
        }

        self.status = None;
    }

    pub fn override_with(&mut self, chess: ChessType) {
        self.status = Some(chess);
    }

    pub fn clear(&mut self) {
        self.status = None;
    }

    pub fn to_string(&self) -> &'static str {
        match self.status {
            Some(chess) => return chess.to_string(),
            None => return "empty",
        }
    }
}
