use std::io;
use std::io::{BufRead, Write};

use colored::{ColoredString, control};
use puzzle::{Color, Corner, Grid, Puzzle};

fn print_puzzle(puzzle: &Puzzle) {
    print!(
        concat!(
            "Goals: {} {} {} {}\n",
            "{}|{}{}{}|{}\n",
            " |{}{}{}| \n",
            "{}|{}{}{}|{}\n"
        ),
        colorize(puzzle.goal(Corner::NW).name(), puzzle.goal(Corner::NW)),
        colorize(puzzle.goal(Corner::NE).name(), puzzle.goal(Corner::NE)),
        colorize(puzzle.goal(Corner::SW).name(), puzzle.goal(Corner::SW)),
        colorize(puzzle.goal(Corner::SE).name(), puzzle.goal(Corner::SE)),
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
        Color::Blue => s.truecolor(0, 0, 255),
    }
}

fn char_to_color(c: char) -> Option<Color> {
    let color = match c {
        '-' => Color::Gray,
        'w' => Color::White,
        'k' => Color::Black,
        'r' => Color::Red,
        'o' => Color::Orange,
        'g' => Color::Green,
        'y' => Color::Yellow,
        'v' => Color::Violet,
        'p' => Color::Pink,
        'b' => Color::Blue,
        _ => return None,
    };
    Some(color)
}

fn parse_puzzle(s: &str) -> Option<Puzzle> {
    let mut colors = s.chars().map(|c| char_to_color(c));
    let goals = [
        colors.next()??,
        colors.next()??,
        colors.next()??,
        colors.next()??,
    ];

    let r2 = [colors.next()??, colors.next()??, colors.next()??];
    let r1 = [colors.next()??, colors.next()??, colors.next()??];
    let r0 = [colors.next()??, colors.next()??, colors.next()??];

    let grid = Grid::from_rows(r2, r1, r0);

    Some(Puzzle::new(goals, grid))
}

fn solve_puzzle(puzzle_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let puzzle = parse_puzzle(&puzzle_str).ok_or("failed to parse puzzle")?;
    print_puzzle(&puzzle);
    let solution = puzzle
        .solve()
        .ok_or("puzzle should always have a solution")?;
    print_solution(&solution);
    Ok(())
}

fn solve_puzzles() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        if let Err(e) = solve_puzzle(&line.unwrap()) {
            eprintln!("{}", e);
        }
    }

    Ok(())
}

fn random_challenge() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating puzzle...");
    let mut puzzle = Puzzle::new_random();
    print_puzzle(&puzzle);
    // let solution = puzzle.solve().expect("puzzle should always have a solution");
    // print_solution(&solution);

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
            "q" => puzzle.press_corner(Corner::NW),
            "w" => puzzle.press_corner(Corner::NE),
            "a" => puzzle.press_corner(Corner::SW),
            "s" => puzzle.press_corner(Corner::SE),
            _ => println!("invalid input"),
        }

        print_puzzle(&puzzle);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    solve_puzzles()
}
