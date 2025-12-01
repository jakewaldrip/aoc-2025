use core::fmt;
use std::{fmt::Display, str::from_utf8};

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum Directions {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

#[allow(dead_code)]
pub const ALL_DIRECTIONS: [Directions; 8] = [
    Directions::Top,
    Directions::TopRight,
    Directions::Right,
    Directions::BottomRight,
    Directions::Bottom,
    Directions::BottomLeft,
    Directions::Left,
    Directions::TopLeft,
];

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub struct Point {
    pub row: usize,
    pub col: usize,
    pub value: u8,
}

pub struct Grid {
    data: Vec<u8>,
    row_size: usize,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(input: &str) -> Self {
        let num_new_lines = input.bytes().filter(|&b| b == b'\n').count();

        let capacity = input.len() - num_new_lines;
        let mut data = Vec::with_capacity(capacity);
        let mut row_size = 0;

        for line in input.lines() {
            let bytes = line.as_bytes();

            if row_size == 0 {
                row_size = bytes.len();
            }

            data.extend_from_slice(bytes);
        }

        Self { data, row_size }
    }

    // get when row size is constant
    pub fn get(&self, row: usize, col: usize) -> Option<&u8> {
        let row_offset = self.row_size * row;

        // Check for out of bounds
        if col >= self.row_size {
            return None;
        }
        if row_offset + col >= self.data.len() {
            return None;
        }

        Some(&self.data[row_offset + col])
    }

    pub fn _as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn _iter_rows(&self) -> impl Iterator<Item = &[u8]> {
        self.data.chunks(self.row_size)
    }

    pub fn iter_2d(&self) -> impl Iterator<Item = Point> {
        self.data.iter().enumerate().map(move |(i, value)| {
            let row = i / self.row_size;
            let col = i % self.row_size;
            Point {
                row,
                col,
                value: *value,
            }
        })
    }

    pub fn get_point_in_direction(&self, point: &Point, direction: &Directions) -> Option<Point> {
        // Map each direction to its row/col offset
        let (dr, dc) = match direction {
            Directions::Top => (-1, 0),
            Directions::TopRight => (-1, 1),
            Directions::Right => (0, 1),
            Directions::BottomRight => (1, 1),
            Directions::Bottom => (1, 0),
            Directions::BottomLeft => (1, -1),
            Directions::Left => (0, -1),
            Directions::TopLeft => (-1, -1),
        };

        // Use checked_add/sub to avoid negative indices
        let new_row = (point.row as isize).checked_add(dr)?;
        let new_col = (point.col as isize).checked_add(dc)?;

        // Guard against negative
        if new_row < 0 || new_col < 0 {
            return None;
        }

        let new_row = new_row as usize;
        let new_col = new_col as usize;

        // Use get and map the result
        self.get(new_row, new_col).map(|val| Point {
            row: new_row,
            col: new_col,
            value: *val,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        let data = &self.data;

        while i < data.len() {
            let line_bytes = &data[i..i + self.row_size];
            i += self.row_size;

            let line = from_utf8(line_bytes).map_err(|_| fmt::Error)?;
            writeln!(f, "{line}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_input() {
        let input = "hello";
        let grid = Grid::new(input);

        // Expected encoding: [len, bytes...]
        let expected = [b'h', b'e', b'l', b'l', b'o'];
        assert_eq!(grid._as_bytes(), &expected);
    }

    #[test]
    fn test_multiple_lines() {
        let input = "abc\ndef";
        let grid = Grid::new(input);

        // "abc" -> [3, a, b, c]
        // "def" -> [3, d, e, f]
        let expected = [b'a', b'b', b'c', b'd', b'e', b'f'];
        assert_eq!(grid._as_bytes(), &expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let grid = Grid::new(input);

        // One line (empty), length = 0
        let expected = [];
        assert_eq!(grid._as_bytes(), &expected);
    }

    #[test]
    fn test_empty_lines() {
        let input = "one\n\ntwo";
        let grid = Grid::new(input);

        // "one" -> [3, o, n, e]
        // ""    -> [0]
        // "two" -> [3, t, w, o]
        let expected = [b'o', b'n', b'e', b't', b'w', b'o'];
        assert_eq!(grid._as_bytes(), &expected);
    }

    #[test]
    fn test_trailing_newline() {
        let input = "foo\nbar\n";
        let grid = Grid::new(input);

        // "foo" -> [3, f, o, o]
        // "bar" -> [3, b, a, r]
        let expected = [b'f', b'o', b'o', b'b', b'a', b'r'];
        assert_eq!(grid._as_bytes(), &expected);
    }

    #[test]
    fn test_nonascii_characters() {
        let input = "héllo\n🌍";
        let grid = Grid::new(input);

        // Bytes:
        // "héllo" -> [6, h, é(0xc3,0xa9), l, l, o]
        // "🌍"    -> [4, F0, 9F, 8C, 8D]
        let expected = [b'h', 0xc3, 0xa9, b'l', b'l', b'o', 0xF0, 0x9F, 0x8C, 0x8D];
        assert_eq!(grid._as_bytes(), &expected);
    }

    #[test]
    fn test_normal_day_input_example() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let expected = [
            b'M', b'M', b'M', b'S', b'X', b'X', b'M', b'A', b'S', b'M', b'M', b'S', b'A', b'M',
            b'X', b'M', b'S', b'M', b'S', b'A', b'A', b'M', b'X', b'S', b'X', b'M', b'A', b'A',
            b'M', b'M',
        ];
        assert_eq!(grid._as_bytes(), &expected);
    }

    #[test]
    fn test_get_first_element() {
        let input = "CMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get(0, 0).unwrap();
        let expected = b'C';
        assert_eq!(found, &expected);
    }

    #[test]
    fn test_get_middle_element() {
        let input = "MMMSXXMASM\nMSAMCMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get(1, 4).unwrap();
        let expected = b'C';
        assert_eq!(found, &expected);
    }

    #[test]
    fn test_get_last_element() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMC\n";
        let grid = Grid::new(input);
        let found = grid.get(2, 9).unwrap();
        let expected = b'C';
        assert_eq!(found, &expected);
    }

    #[test]
    fn get_out_of_row_bounds() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get(5, 5);
        let expected = None;
        assert_eq!(found, expected);
    }

    #[test]
    fn get_out_of_col_bounds() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get(1, 11);
        let expected = None;
        assert_eq!(found, expected);
    }

    #[test]
    fn get_point_in_direction_top() {
        let input = "MCMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get_point_in_direction(
            &Point {
                row: 1,
                col: 1,
                value: b'C',
            },
            &Directions::Top,
        );

        let expected = Some(Point {
            row: 0,
            col: 1,
            value: b'C',
        });
        assert_eq!(found, expected);
    }

    #[test]
    fn get_point_in_direction_bottom() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nACXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get_point_in_direction(
            &Point {
                row: 1,
                col: 1,
                value: b'C',
            },
            &Directions::Bottom,
        );

        let expected = Some(Point {
            row: 2,
            col: 1,
            value: b'C',
        });
        assert_eq!(found, expected);
    }

    #[test]
    fn get_point_in_direction_left() {
        let input = "MMMSXXMASM\nCSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get_point_in_direction(
            &Point {
                row: 1,
                col: 1,
                value: b'C',
            },
            &Directions::Left,
        );

        let expected = Some(Point {
            row: 1,
            col: 0,
            value: b'C',
        });
        assert_eq!(found, expected);
    }

    #[test]
    fn get_point_in_direction_right() {
        let input = "MMMSXXMASM\nMSCMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get_point_in_direction(
            &Point {
                row: 1,
                col: 1,
                value: b'C',
            },
            &Directions::Right,
        );

        let expected = Some(Point {
            row: 1,
            col: 2,
            value: b'C',
        });
        assert_eq!(found, expected);
    }

    #[test]
    fn get_point_in_direction_out_of_bounds() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get_point_in_direction(
            &Point {
                row: 0,
                col: 0,
                value: b'C',
            },
            &Directions::Top,
        );

        assert_eq!(found, None);
    }

    #[test]
    fn get_point_in_direction_out_of_bounds_overflow() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\n";
        let grid = Grid::new(input);
        let found = grid.get_point_in_direction(
            &Point {
                row: 2,
                col: 8,
                value: b'C',
            },
            &Directions::BottomRight,
        );

        assert_eq!(found, None);
    }

    #[test]
    fn get_point_in_direction_right_side() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

        let grid = Grid::new(input);
        let found = grid.get_point_in_direction(
            &Point {
                row: 7,
                col: 9,
                value: b'A',
            },
            &Directions::BottomRight,
        );

        assert_eq!(found, None);
    }
}
