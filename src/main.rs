extern crate ansi_term;

use std::collections::HashSet;
use std::io::Read;

mod maze;
mod point;

use maze::*;
use point::*;

fn main() {
    let maze_txt = {
        let mut buf = String::new();
        std::fs::File::open("maze2.txt")
            .unwrap()
            .read_to_string(&mut buf)
            .unwrap();
        buf
    };

    let mut maze = Maze::new(&maze_txt);

    if let Some(mut path) = walk(maze.start, &maze, &mut HashSet::new()) {
        path.reverse();
        for i in 1..path.len() {
            for m in &path[0..i] {
                maze[m.1][m.0] = 2;
            }
            maze.render();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("Number of moves: {}", path.len());
    }
}

fn walk(from: Point, maze: &Maze, visited: &mut HashSet<Point>) -> Option<Vec<Point>> {
    if from == maze.end {
        return Some(vec![from]);
    }

    let moves: HashSet<_> = maze
        .valid_moves(from)
        .difference(visited)
        .cloned()
        .collect();

    for m in moves {
        visited.insert(m);
        if let Some(mut v) = walk(m, maze, visited) {
            v.push(from);
            return Some(v);
        }
    }

    None
}
