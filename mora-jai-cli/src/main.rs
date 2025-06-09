use std::io::Write;

use colored::ColoredString;
use puzzle::{Color, Corner, Puzzle};

fn print_puzzle(puzzle: &Puzzle) {
    print!(
        concat!("Goal: {}\n", "{}|{}{}{}|{}\n", " |{}{}{}| \n", "{}|{}{}{}|{}\n"),
        colorize(puzzle.goal().name(), puzzle.goal()),
        colorize("q", puzzle.get_corner(Corner::NW)),
        colorize("7", puzzle.get_tile(2, 0)),
        colorize("8", puzzle.get_tile(2, 1)),
        colorize("9", puzzle.get_tile(2, 2)),
        colorize("w", puzzle.get_corner(Corner::NE)),
        colorize("4", puzzle.get_tile(1, 0)),
        colorize("5", puzzle.get_tile(1, 1)),
        colorize("6", puzzle.get_tile(1, 2)),
        colorize("a", puzzle.get_corner(Corner::SW)),
        colorize("1", puzzle.get_tile(0, 0)),
        colorize("2", puzzle.get_tile(0, 1)),
        colorize("3", puzzle.get_tile(0, 2)),
        colorize("s", puzzle.get_corner(Corner::SE)),
    );
}

fn print_solution(solution: &[(usize, usize)]) {
    print!("Solution: ");
    for (row, col) in solution {
        let num = 1 + 3 * row + col;
        print!("{} ", num);
    }
    println!();
}

fn colorize(s: &str, color: Color) -> ColoredString {
    // Import here to avoid adding .blue(), .red(), etc. methods to all strings
    use colored::Colorize;

    match color {
        Color::Gray => s.truecolor(128, 128, 128),
        Color::White => s.white(),
        Color::Black => s.truecolor(0, 0, 0).on_truecolor(64, 64, 64),
        Color::Red => s.truecolor(255, 0, 0),
        Color::Pink => s.truecolor(255, 192, 203),
        Color::Green => s.truecolor(0, 255, 0),
        Color::Orange => s.truecolor(255, 165, 0),
        Color::Yellow => s.truecolor(255, 255, 0),
        Color::Violet => s.truecolor(127, 0, 255),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut puzzle = Puzzle::new_random();
    print_puzzle(&puzzle);
    let solution = puzzle.solve().expect("puzzle should always have a solution");
    print_solution(&solution);

    while !puzzle.is_solved() {
        print!("Input: ");
        std::io::stdout().flush()?;
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;

        match line.trim() {
            "1" => puzzle.press_tile(0, 0),
            "2" => puzzle.press_tile(0, 1),
            "3" => puzzle.press_tile(0, 2),
            "4" => puzzle.press_tile(1, 0),
            "5" => puzzle.press_tile(1, 1),
            "6" => puzzle.press_tile(1, 2),
            "7" => puzzle.press_tile(2, 0),
            "8" => puzzle.press_tile(2, 1),
            "9" => puzzle.press_tile(2, 2),
            _ => todo!(),
        }

        print_puzzle(&puzzle);
    }

    Ok(())
}
