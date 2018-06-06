#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn left(&self) -> Option<Point> {
        self.0.checked_sub(1).map(|x| Point(x, self.1))
    }
    fn right(&self) -> Option<Point> {
        self.0.checked_add(1).map(|x| Point(x, self.1))
    }
    fn up(&self) -> Option<Point> {
        self.1.checked_sub(1).map(|y| Point(self.0, y))
    }
    fn down(&self) -> Option<Point> {
        self.1.checked_add(1).map(|y| Point(self.0, y))
    }
}

#[derive(Debug)]
struct Maze {
    layout: Vec<u8>,
    width: usize,
    start: Point,
    end: Point,
}

use std::collections::HashSet;
use std::ops::{Index, IndexMut};

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
    fn new(maze_txt: &str) -> Maze {
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

    fn is_wall(&self, pos: Point) -> bool {
        if pos.0 >= self.width || pos.1 >= self.width {
            true
        } else {
            self[pos.1][pos.0] == 0
        }
    }

    fn valid_moves(&self, from: Point) -> HashSet<Point> {
        let moves = [from.left(), from.right(), from.up(), from.down()];
        moves        
            .into_iter()
            .filter_map(|p| *p)
            .filter(|p| !self.is_wall(*p))
            .collect()
    }
    
    fn render(&self) {
        for y in 0..self.width {
            for x in 0..self.width {
                print!(
                    "{}",
                    match self[y][x] {
                        0 => "■ ",
                        1 if self.start == Point(x, y) => "..",
                        1 if self.end == Point(x, y) => "''",
                        1 => "  ",
                        2 => ". ",
                        _ => panic!("unexpected value"),
                    }
                );
            }
            println!();
        }
        println!();
    }
}

use std::io::Read;

fn main() {
    let maze_txt = {
        let mut buf = String::new();
        std::fs::File::open("maze.txt").unwrap().read_to_string(&mut buf).unwrap();
        buf
    };
    let mut maze = Maze::new(&maze_txt);
    if let Some(mut path) = walk(maze.start, &maze, &mut HashSet::new()) {
        path.reverse();
        for m in &path {
            maze[m.1][m.0] = 2;
        }
        maze.render();
        println!("Number of moves: {}", path.len());
    }
}

fn walk(from: Point, maze: &Maze, visited: &mut HashSet<Point>) -> Option<Vec<Point>> {
    if from == maze.end {
        return Some(vec![from]);
    }

    let moves: HashSet<_> = maze.valid_moves(from).difference(visited).cloned().collect();

    for m in moves {
        visited.insert(m);
        if let Some(mut v) = walk(m, maze, visited) {
            v.push(from);
            return Some(v);
        }
    }

    None
}
