use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Color {
    Gray,
    White,
    Black,
    Red,
    Orange,
    Green,
    Yellow,
    Violet,
    Pink,
    // Don't forget to update num_variants() after adding a color
}

impl Color {
    pub fn num_variants() -> usize {
        9
    }

    pub fn name(&self) -> &'static str {
        match self {
            Color::Gray => "gray",
            Color::White => "white",
            Color::Black => "black",
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Green => "green",
            Color::Yellow => "yellow",
            Color::Violet => "violet",
            Color::Pink => "pink",
        }
    }
}

/// A Mora Jai puzzle's grid.
///
/// The row, column pairs of each tile are as follows.
/// -------------------
/// | 2,0 | 2,1 | 2,2 |
/// | 1,0 | 1,1 | 1,2 |
/// | 0,0 | 0,1 | 0,2 |
/// -------------------
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid {
    colors: [Color; 9],
}

impl Grid {
    pub fn new(colors: impl Into<[Color; 9]>) -> Self {
        Self {
            colors: colors.into(),
        }
    }

    /// Convenience function to build Mora Jai puzzle grids
    pub fn from_rows(r2: [Color; 3], r1: [Color; 3], r0: [Color; 3]) -> Self {
        let colors = [
            r0[0].clone(),
            r0[1].clone(),
            r0[2].clone(),
            r1[0].clone(),
            r1[1].clone(),
            r1[2].clone(),
            r2[0].clone(),
            r2[1].clone(),
            r2[2].clone(),
        ];
        Self::new(colors)
    }

    fn valid_coord(row: usize, col: usize) -> bool {
        row < 3 && col < 3
    }

    /// Returns the color at the given row and column.
    /// Rows and columns must be 0, 1, or 2.
    /// Panics if the row or column is invalid.
    pub fn get(&self, row: usize, col: usize) -> &Color {
        if !Self::valid_coord(row, col) {
            panic!("invalid row or column");
        }

        let idx: usize = row * 3 + col;
        &self.colors[idx]
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Color {
        if !Self::valid_coord(row, col) {
            panic!("invalid row or column");
        }

        let idx: usize = row * 3 + col;
        &mut self.colors[idx]
    }

    fn neighbours_clockwise(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        if !Self::valid_coord(row, col) {
            panic!("invalid row or column");
        }

        let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(8);
        let offsets: [(isize, isize); 8] = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        for (dr, dc) in offsets {
            let row = row.checked_add_signed(dr).unwrap_or(usize::MAX);
            let col = col.checked_add_signed(dc).unwrap_or(usize::MAX);

            if Self::valid_coord(row, col) {
                neighbours.push((row, col));
            }
        }

        neighbours
    }

    /// Press a tile on this puzzle. The resulting puzzle is returned.
    pub fn press(&self, row: usize, col: usize) -> Self {
        let color = self.get(row, col);

        let mut copy = self.clone();

        match color {
            // Gray tiles do nothing
            Color::Gray => {}
            // White tiles toggle themselves and all orthogonally adjacent white or gray tiles
            Color::White => {
                let mut adjacent: Vec<(usize, usize)> = Vec::with_capacity(5);
                adjacent.push((row, col));
                if row > 0 {
                    adjacent.push((row - 1, col));
                }
                if row < 2 {
                    adjacent.push((row + 1, col));
                }
                if col > 0 {
                    adjacent.push((row, col - 1));
                }
                if col < 2 {
                    adjacent.push((row, col + 1));
                }

                for (row, col) in adjacent.into_iter() {
                    match self.get(row, col) {
                        Color::White => *copy.get_mut(row, col) = Color::Gray,
                        Color::Gray => *copy.get_mut(row, col) = Color::White,
                        _ => {}
                    }
                }
            }
            // Black tiles rotate a row to the right
            Color::Black => {
                for col in 0..3 {
                    // Index of column directly to the right of col, wrapping if necessary.
                    let right_col = (col + 1) % 3;
                    *copy.get_mut(row, right_col) = self.get(row, col).clone();
                }
            }
            // All black tiles become red and all white tiles become black
            Color::Red => {
                for row in 0..3 {
                    for col in 0..3 {
                        match self.get(row, col) {
                            Color::Black => *copy.get_mut(row, col) = Color::Red,
                            Color::White => *copy.get_mut(row, col) = Color::Black,
                            _ => {}
                        }
                    }
                }
            }
            // If there is a majority color among the orthogonal neighbours, this tile becomes that color
            Color::Orange => {
                let mut adjacent: Vec<(usize, usize)> = Vec::with_capacity(4);
                if row > 0 {
                    adjacent.push((row - 1, col));
                }
                if row < 2 {
                    adjacent.push((row + 1, col));
                }
                if col > 0 {
                    adjacent.push((row, col - 1));
                }
                if col < 2 {
                    adjacent.push((row, col + 1));
                }

                let mut counts: BTreeMap<Color, u8> = Default::default();
                for (row, col) in adjacent.into_iter() {
                    let color = self.get(row, col);
                    *counts.entry(color.clone()).or_insert(0) += 1;
                }

                let max = *counts.values().max().expect("map should never be empty");

                let max_colors: Vec<Color> = counts
                    .into_iter()
                    .filter(|&(_, count)| count == max)
                    .map(|(color, _)| color)
                    .collect();

                // If only one color has the maximum, it is the majority color
                if max_colors.len() == 1 {
                    let majority = max_colors[0].clone();
                    *copy.get_mut(row, col) = majority;
                }
            }
            // Green tiles swap with the opposite tile
            Color::Green => {
                let opposing_row = 2 - row;
                let opposing_col = 2 - col;
                *copy.get_mut(opposing_row, opposing_col) = self.get(row, col).clone();
                *copy.get_mut(row, col) = self.get(opposing_row, opposing_col).clone();
            }
            // Yellow tiles swap with the tile directly above, or do nothing if they are
            // at the top
            Color::Yellow => {
                if row < 2 {
                    let upper_row = row + 1;
                    *copy.get_mut(upper_row, col) = self.get(row, col).clone();
                    *copy.get_mut(row, col) = self.get(upper_row, col).clone();
                }
            }
            // Violet tiles swap with the tile directly below, or do nothing if they are
            // at the bottom
            Color::Violet => {
                if row > 0 {
                    let lower_row = row - 1;
                    *copy.get_mut(lower_row, col) = self.get(row, col).clone();
                    *copy.get_mut(row, col) = self.get(lower_row, col).clone();
                }
            }
            // Pink tiles rotate their neighbours (including diagonals) clockwise.
            Color::Pink => {
                let neighbours = self.neighbours_clockwise(row, col);
                for window in neighbours.windows(2) {
                    let first = window[0];
                    let second = window[1];

                    *copy.get_mut(second.0, second.1) = self.get(first.0, first.1).clone();
                }
            }
        }

        copy
    }
}

pub enum Corner {
    NE,
    SE,
    SW,
    NW,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puzzle {
    goal: Color,
    pub(super) corners: [Color; 4],
    /// The original state of the puzzle grid, used for resets
    pub(super) original: Grid,
    /// Current state of the puzzle grid
    state: Grid,
}

impl Puzzle {
    pub fn new(goal: Color, grid: Grid) -> Self {
        Self {
            goal,
            corners: [const { Color::Gray }; 4],
            original: grid.clone(),
            state: grid,
        }
    }

    pub fn current_state(&self) -> &Grid {
        &self.state
    }

    pub fn goal(&self) -> Color {
        self.goal
    }
    
    pub fn get_tile(&self, row: usize, col: usize) -> Color {
        *self.current_state().get(row, col)
    }

    pub fn get_corner(&self, corner: Corner) -> Color {
        match corner {
            Corner::SW => self.corners[0],
            Corner::NW => self.corners[1],
            Corner::SE => self.corners[2],
            Corner::NE => self.corners[3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_works() {
        let puzzle = Grid::from_rows(
            [Color::Gray, Color::Gray, Color::Gray],
            [Color::Gray, Color::Gray, Color::Gray],
            [Color::Gray, Color::Gray, Color::Gray],
        );

        for row in 0..3 {
            for col in 0..3 {
                let new = puzzle.press(row, col);
                assert_eq!(puzzle, new);
            }
        }
    }

    #[test]
    fn white_center_works() {
        let puzzle = Grid::from_rows(
            [Color::Gray, Color::Gray, Color::Gray],
            [Color::Gray, Color::White, Color::Gray],
            [Color::Gray, Color::Gray, Color::Gray],
        );

        let new = puzzle.press(1, 1);
        assert_eq!(
            new,
            Grid::from_rows(
                [Color::Gray, Color::White, Color::Gray],
                [Color::White, Color::Gray, Color::White],
                [Color::Gray, Color::White, Color::Gray],
            )
        );
    }

    #[test]
    fn white_corner_works() {
        let puzzle = Grid::from_rows(
            [Color::Gray, Color::Gray, Color::Gray],
            [Color::Gray, Color::Gray, Color::Gray],
            [Color::White, Color::Gray, Color::Gray],
        );

        let new = puzzle.press(0, 0);
        assert_eq!(
            new,
            Grid::from_rows(
                [Color::Gray, Color::Gray, Color::Gray],
                [Color::White, Color::Gray, Color::Gray],
                [Color::Gray, Color::White, Color::Gray],
            )
        );
    }

    #[test]
    fn black_works() {
        let puzzle = Grid::from_rows(
            [Color::Gray, Color::Gray, Color::Gray],
            [Color::Gray, Color::Gray, Color::Gray],
            [Color::Black, Color::White, Color::Red],
        );

        let new = puzzle.press(0, 0);
        assert_eq!(
            new,
            Grid::from_rows(
                [Color::Gray, Color::Gray, Color::Gray],
                [Color::Gray, Color::Gray, Color::Gray],
                [Color::Red, Color::Black, Color::White]
            ),
        );

        let new = new.press(0, 1);
        assert_eq!(
            new,
            Grid::from_rows(
                [Color::Gray, Color::Gray, Color::Gray],
                [Color::Gray, Color::Gray, Color::Gray],
                [Color::White, Color::Red, Color::Black]
            ),
        );

        let new = new.press(0, 2);
        assert_eq!(puzzle, new);
    }

    #[test]
    fn red_works() {
        let puzzle = Grid::from_rows(
            [Color::White, Color::White, Color::White],
            [Color::White, Color::Red, Color::Black],
            [Color::Black, Color::Black, Color::Black],
        );

        let new = puzzle.press(1, 1);
        assert_eq!(
            new,
            Grid::from_rows(
                [Color::Black, Color::Black, Color::Black],
                [Color::Black, Color::Red, Color::Red],
                [Color::Red, Color::Red, Color::Red],
            ),
        );
    }
}
