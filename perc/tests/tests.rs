use perc::{evaluate_probability, percolates, BoolGrid};

////////////////////////////////////////////////////////////////////////////////

fn make_grid(text: &str) -> BoolGrid {
    let lines = text
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    if lines.is_empty() {
        return BoolGrid::new(0, 0);
    }

    let mut grid = BoolGrid::new(lines[0].len(), lines.len());
    for (y, line) in lines.into_iter().enumerate() {
        assert_eq!(line.len(), grid.width());
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                grid.set(x, y, true);
            } else if c != '.' {
                panic!("Unexpected char: {}", c);
            }
        }
    }

    grid
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_grid_basics() {
    let mut grid = BoolGrid::new(3, 5);
    assert_eq!(grid.width(), 3);
    assert_eq!(grid.height(), 5);

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            assert!(!grid.get(x, y));
        }
    }

    grid.set(2, 4, true);
    assert!(grid.get(2, 4));
    grid.set(2, 4, false);
    assert!(!grid.get(2, 4));
}

#[test]
fn test_custom_grid() {
    let grid = make_grid(
        "
        ##..##.
        ###.#.#
        ",
    );
    assert_eq!(grid.width(), 7);
    assert_eq!(grid.height(), 2);
    assert!(!grid.get(2, 0));
    assert!(grid.get(6, 1));
}

#[test]
fn test_percolates() {
    assert!(percolates(&make_grid(
        "
            ###.###
            #....##
            ##.#.##
            ####.##
        "
    )));
    assert!(!percolates(&make_grid(
        "
            ###.###
            #....##
            ##.####
            ####.##
        "
    )));
    assert!(percolates(&BoolGrid::new(0, 125)));
    assert!(percolates(&BoolGrid::new(235, 0)));
    assert!(percolates(&BoolGrid::new(0, 0)));
    assert!(percolates(&BoolGrid::random(50, 50, 0.9)));
}

#[test]
fn test_probability() {
    for (width, height, vacancy, expected) in
        [(10, 10, 0.57, 0.425), (5, 8, 0.6, 0.35), (3, 4, 0.5, 0.25)]
    {
        let actual = evaluate_probability(width, height, vacancy);
        assert!(
            (expected - actual).abs() < 0.02,
            "wrong answer: expected {} +- 0.02, got {} (width: {}, height: {}, vacancy: {})",
            expected,
            actual,
            width,
            height,
            vacancy,
        );
    }
}
