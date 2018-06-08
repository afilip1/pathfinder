extern crate ansi_term;

mod maze;
mod point;

use maze::*;
use point::*;

use std::{thread, collections::HashSet, time::Duration};

fn main() {
    let maze_txt = include_str!("../maze2.txt");

    let mut maze = Maze::new(&maze_txt);

    if let Some(mut path) = walk(maze.start, &maze, &mut HashSet::new()) {
        path.reverse();
        for i in 1..path.len() {
            for m in &path[0..i] {
                maze[m.1][m.0] = 2;
            }
            maze.render();
            thread::sleep(Duration::from_millis(100));
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
