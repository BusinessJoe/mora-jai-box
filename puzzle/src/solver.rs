use std::collections::{HashSet, VecDeque};

use rand::distr::{Distribution, StandardUniform};

use crate::{
    puzzle::{Color, Grid},
    Puzzle,
};

fn is_solved(goal: &Color, grid: &Grid) -> bool {
    grid.get(0, 0) == goal
        && grid.get(0, 2) == goal
        && grid.get(2, 0) == goal
        && grid.get(2, 2) == goal
}

/// Search for a solution to a Mora Jai puzzle.
///
/// Returns a sequence of coordinates that corresponds to the solution's button presses
/// or None if no solution exists.
fn solve(goal: &Color, grid: &Grid) -> Option<Vec<(usize, usize)>> {
    type Solution = (Grid, Vec<(usize, usize)>);

    let start = (grid.clone(), vec![]);
    let mut queue: VecDeque<Solution> = VecDeque::from([start]);
    let mut seen: HashSet<Grid> = Default::default();

    while let Some((grid, path)) = queue.pop_front() {
        if seen.contains(&grid) {
            continue;
        } else {
            seen.insert(grid.clone());
        }

        if is_solved(goal, &grid) {
            return Some(path);
        }

        for row in 0..3 {
            for col in 0..3 {
                let new_grid = grid.press(row, col);
                let mut new_path = path.clone();
                new_path.push((row, col));

                queue.push_back((new_grid, new_path));
            }
        }
    }

    None
}

impl Distribution<Color> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.random_range(0..Color::num_variants()) {
            0 => Color::Gray,
            1 => Color::White,
            2 => Color::Black,
            3 => Color::Red,
            4 => Color::Orange,
            5 => Color::Green,
            6 => Color::Yellow,
            7 => Color::Violet,
            8 => Color::Pink,
            _ => unreachable!(),
        }
    }
}

impl Puzzle {
    pub fn new_random() -> Self {
        // Randomly generate puzzles until we find one with a solution
        loop {
            let goal: Color = rand::random();
            let colors: [Color; 9] = rand::random();
            let grid = Grid::new(colors);

            if solve(&goal, &grid).is_some() {
                return Self::new(goal, grid);
            }
        }
    }

    pub fn solve(&self) -> Option<Vec<(usize, usize)>> {
        solve(&self.goal(), &self.original)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        let grid = Grid::from_rows(
            [Color::White, Color::White, Color::White],
            [Color::White, Color::Gray, Color::White],
            [Color::Gray, Color::Gray, Color::White],
        );

        let solution = solve(&Color::White, &grid);

        assert_eq!(Some(vec![(0, 2), (0, 1)]), solution);
    }
}
