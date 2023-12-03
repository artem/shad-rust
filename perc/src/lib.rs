#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

/// Represents a grid of boolean values.
pub struct BoolGrid {
    // TODO: your code here.
}

impl BoolGrid {
    /// Creates a new grid with all values initialized as `false`.
    ///
    /// # Arguments
    ///
    /// * `width` - grid width.
    /// * `height` - grid height.
    pub fn new(width: usize, height: usize) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    /// Creates a new grid with every value initialized randomly.
    ///
    /// # Arguments
    ///
    /// * `width` - grid width.
    /// * `height` - grid height.
    /// * `vacancy` - probability of any given value being equal
    /// to `false`.
    pub fn random(width: usize, height: usize, vacancy: f64) -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    /// Returns grid width.
    pub fn width(&self) -> usize {
        // TODO: your code here.
        unimplemented!()
    }

    /// Returns grid height.
    pub fn height(&self) -> usize {
        // TODO: your code here.
        unimplemented!()
    }

    /// Returns the current value of a given cell.
    /// The caller must ensure that `x` and `y` are valid.
    ///
    /// # Arguments
    ///
    /// * `x` - must be >= 0 and < grid width.
    /// * `y` - must be >= 0 and < grid height.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is out of bounds, this method may panic
    /// (or return incorrect result).
    pub fn get(&self, x: usize, y: usize) -> bool {
        // TODO: your code here.
        unimplemented!()
    }

    /// Sets a new value to a given cell.
    /// The caller must ensure that `x` and `y` are valid.
    ///
    /// # Arguments
    ///
    /// * `x` - must be >= 0 and < grid width.
    /// * `y` - must be >= 0 and < grid height.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is out of bounds, this method may panic
    /// (or set value to some other unspecified cell).
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        // TODO: your code here.
        unimplemented!()
    }

    // TODO: your code here.
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////

/// Returns `true` if the given grid percolates. That is, if there is a path
/// from any cell with `y` == 0 to any cell with `y` == `height` - 1.
/// If the grid is empty (`width` == 0 or `height` == 0), it percolates.
pub fn percolates(grid: &BoolGrid) -> bool {
    // TODO: your code here.
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////

const N_TRIALS: u64 = 10000;

/// Returns an estimate of the probability that a random grid with given
/// `width, `height` and `vacancy` probability percolates.
/// To compute an estimate, it runs `N_TRIALS` of random experiments,
/// in each creating a random grid and checking if it percolates.
pub fn evaluate_probability(width: usize, height: usize, vacancy: f64) -> f64 {
    let mut perc_count = 0;
    for _ in 0..N_TRIALS {
        let grid = BoolGrid::random(width, height, vacancy);
        if percolates(&grid) {
            perc_count += 1;
        }
    }
    return perc_count as f64 / N_TRIALS as f64;
}
