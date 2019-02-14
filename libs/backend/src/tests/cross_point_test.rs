use super::super::cross_point::*;

#[test]
fn get_and_set() {
    // create by new()
    let cp = CrossPoint::new();
    assert_eq!(cp.have_chess(), false);
    assert_eq!(cp.get_chess(), None);

    // create_by_chess() black
    let mut cp = CrossPoint::create_by_chess(ChessType::CtBlack);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), Some(ChessType::CtBlack));

    // create_by_chess() white
    let mut cp = CrossPoint::create_by_chess(ChessType::CtWhite);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), Some(ChessType::CtWhite));

    // remove
    cp.remove_chess();
    assert_eq!(cp.have_chess(), false);
    assert_eq!(cp.get_chess(), None);

    // put chess black
    cp.put_chess(ChessType::CtBlack);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), Some(ChessType::CtBlack));

    // clear()
    cp.clear();
    assert_eq!(cp.have_chess(), false);
    assert_eq!(cp.get_chess(), None);

    // put chess white
    cp.put_chess(ChessType::CtWhite);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), Some(ChessType::CtWhite));

    // override with black
    cp.override_with(ChessType::CtBlack);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), Some(ChessType::CtBlack));

    // override with white
    cp.override_with(ChessType::CtWhite);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), Some(ChessType::CtWhite));
}

#[test]
#[should_panic]
fn remove_chess_panic() {
    let mut cp = CrossPoint::new();
    cp.remove_chess();
}

#[test]
#[should_panic]
fn put_chess_panic_black() {
    let mut cp = CrossPoint::create_by_chess(ChessType::CtBlack);
    cp.put_chess(ChessType::CtWhite);
}

#[test]
#[should_panic]
fn put_chess_panic_white() {
    let mut cp = CrossPoint::create_by_chess(ChessType::CtWhite);
    cp.put_chess(ChessType::CtBlack);
}
