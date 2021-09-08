#![forbid(unsafe_code)]

fn main() {
    let mut input = String::new();
    for _ in 0..3 {
        std::io::stdin().read_line(&mut input).unwrap();
    }

    let board = puzzle::Board::from_string(&input);
    if let Some(moves) = puzzle::solve(board) {
        for mv in moves {
            print!("---\n{}", mv.to_string());
        }
    } else {
        println!("No solution.");
    }
}
