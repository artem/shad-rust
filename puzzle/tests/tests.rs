use puzzle::{solve, Board, Tile};

#[test]
fn test_tile_basics() {
    assert!(!Tile::new(Some(4)).is_empty());
    assert!(Tile::new(None).is_empty());
    assert!(Tile::empty().is_empty());
    assert_eq!(Tile::new(Some(8)).number(), Some(8));
    assert_eq!(Tile::empty().number(), None);
    assert_eq!(Tile::new(None).number(), None);
}

#[test]
#[should_panic]
fn test_tile_0() {
    Tile::new(Some(0));
}

#[test]
#[should_panic]
fn test_tile_9() {
    Tile::new(Some(9));
}

#[test]
fn test_board_basics() {
    let mut board = Board::new([
        [Tile::new(Some(3)), Tile::new(Some(2)), Tile::new(Some(1))],
        [Tile::new(Some(6)), Tile::new(Some(5)), Tile::new(Some(4))],
        [Tile::empty(), Tile::new(Some(7)), Tile::new(Some(8))],
    ]);
    assert_eq!(board.get(0, 0), Tile::new(Some(3)));
    assert_eq!(board.get(1, 2), Tile::new(Some(4)));

    board.swap(2, 0, 0, 1);
    assert_eq!(board.get(2, 0), Tile::new(Some(2)));
    assert_eq!(board.get(0, 1), Tile::empty());

    assert_eq!(
        board.to_string(),
        "3.1\n\
         654\n\
         278\n",
    );

    assert_eq!(
        board,
        Board::from_string(
            "3.1\n\
             654\n\
             278\n"
        ),
    );
}

#[test]
#[should_panic]
fn test_board_illegal() {
    Board::new([
        [Tile::new(Some(3)), Tile::new(Some(2)), Tile::new(Some(1))],
        [Tile::new(Some(6)), Tile::new(Some(5)), Tile::new(Some(4))],
        [Tile::empty(), Tile::new(Some(7)), Tile::new(Some(2))],
    ]);
}

#[test]
#[should_panic]
fn test_board_illegal_from_string_1() {
    Board::from_string(
        "012\n\
         456\n\
         78.\n",
    );
}

#[test]
#[should_panic]
fn test_board_illegal_from_string_2() {
    Board::from_string(
        "...\n\
         .1.\n\
         ...\n",
    );
}

#[test]
fn test_solve() {
    assert_eq!(
        solve(Board::from_string(
            "123\n\
             456\n\
             78.\n"
        )),
        Some(vec![]),
    );
    assert_eq!(
        solve(Board::from_string(
            "123\n\
             456\n\
             7.8\n"
        )),
        Some(vec![Board::from_string(
            "123\n\
             456\n\
             78.\n"
        ),]),
    );
    assert_eq!(
        solve(Board::from_string(
            "12.\n\
             453\n\
             786\n"
        )),
        Some(vec![
            Board::from_string(
                "123\n\
                 45.\n\
                 786\n"
            ),
            Board::from_string(
                "123\n\
                 456\n\
                 78.\n"
            ),
        ]),
    );
    assert_eq!(
        solve(Board::from_string(
            "123\n\
             456\n\
             87.\n"
        )),
        None,
    );

    let solution = solve(Board::from_string(
        "321\n\
         654\n\
         .78\n",
    ));
    assert!(solution.is_some());
    assert_eq!(solution.unwrap().len(), 24);
}
