use point::*;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Maze {
    layout: Vec<u8>,
    width: usize,
    pub start: Point,
    pub end: Point,
}

impl Index<usize> for Maze {
    type Output = [u8];

    fn index(&self, idx: usize) -> &[u8] {
        let from = idx * self.width;
        let to = (idx + 1) * self.width;
        &self.layout[from..to]
    }
}

impl IndexMut<usize> for Maze {
    fn index_mut(&mut self, idx: usize) -> &mut [u8] {
        let from = idx * self.width;
        let to = (idx + 1) * self.width;
        &mut self.layout[from..to]
    }
}

impl Maze {
    pub fn new(maze_txt: &str) -> Maze {
        let mut layout = vec![];
        let mut width = 0;
        let mut start = 0;
        let mut end = 0;
        for c in maze_txt.chars() {
            match c {
                '@' => {
                    start = layout.len();
                    layout.push(1);
                }
                '$' => {
                    end = layout.len();
                    layout.push(1);
                }
                '\n' => match width {
                    0 => width = layout.len(),
                    _ => assert!(layout.len() % width == 0),
                },
                '.' => layout.push(1),
                'x' => layout.push(0),
                _ => continue,
            }
        }
        Maze {
            layout,
            width,
            start: Point(start % width, start / width),
            end: Point(end % width, end / width),
        }
    }

    pub fn is_wall(&self, pos: Point) -> bool {
        if pos.0 >= self.width || pos.1 >= self.width {
            true
        } else {
            self[pos.1][pos.0] == 0
        }
    }

    pub fn valid_moves(&self, from: Point) -> HashSet<Point> {
        from.neighbors()
            .into_iter()
            .filter_map(|p| p)
            .filter(|p| !self.is_wall(*p))
            .collect()
    }

    pub fn render(&self) {
        use std;
        use std::io::Write;
        use ansi_term::Colour::{Fixed, Red};
        let dot = Red.paint(". ").to_string();
        let wall = Fixed(238).paint("■ ").to_string();
        let mut out_buf = String::new();

        for y in 0..self.width {
            for x in 0..self.width {
                out_buf.push_str(
                    match self[y][x] {
                        0 => &wall,
                        1 if self.start == Point(x, y) => "..",
                        1 if self.end == Point(x, y) => "''",
                        1 => "  ",
                        2 => &dot,
                        _ => panic!("unexpected value"),
                    }
                );
            }
            out_buf.push('\n');
        }
        out_buf.push('\n');
        println!("{}", out_buf);
        std::io::stdout().flush().unwrap();
    }
}
