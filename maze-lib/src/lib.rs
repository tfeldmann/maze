extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod unicode;

type Dir = u8;

struct Point {
    x: i32,
    y: i32,
}

struct Direction {
    passage: Dir,
    opposite: Dir,
    delta: Point,
}

pub const UP: Dir = 0b0001;
pub const RIGHT: Dir = 0b0010;
pub const DOWN: Dir = 0b0100;
pub const LEFT: Dir = 0b1000;

/*
All direction permutations for BFS
[
    (0, 1, 2, 3),
    (0, 1, 3, 2),
    (0, 2, 1, 3),
    (0, 2, 3, 1),
    (0, 3, 1, 2),
    (0, 3, 2, 1),
    (1, 0, 2, 3),
    (1, 0, 3, 2),
    (1, 2, 0, 3),
    (1, 2, 3, 0),
    (1, 3, 0, 2),
    (1, 3, 2, 0),
    (2, 0, 1, 3),
    (2, 0, 3, 1),
    (2, 1, 0, 3),
    (2, 1, 3, 0),
    (2, 3, 0, 1),
    (2, 3, 1, 0),
    (3, 0, 1, 2),
    (3, 0, 2, 1),
    (3, 1, 0, 2),
    (3, 1, 2, 0),
    (3, 2, 0, 1),
    (3, 2, 1, 0)
]
*/

const DIRECTIONS: [Direction; 4] = [
    Direction {
        passage: UP,
        opposite: DOWN,
        delta: Point { x: 0, y: -1 },
    },
    Direction {
        passage: RIGHT,
        opposite: LEFT,
        delta: Point { x: 1, y: 0 },
    },
    Direction {
        passage: DOWN,
        opposite: UP,
        delta: Point { x: 0, y: 1 },
    },
    Direction {
        passage: LEFT,
        opposite: RIGHT,
        delta: Point { x: -1, y: 0 },
    },
];

#[derive(Debug)]
pub struct Maze {
    height: usize,
    width: usize,
    path: Vec<Vec<Dir>>,
}

impl Maze {
    // Algorithm based on:
    // https://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm
    pub fn growing_tree(width: usize, height: usize) -> Self {
        let mut path: Vec<Vec<Dir>> = vec![vec![0; width]; height];
        let mut visited: Vec<Point> = Vec::with_capacity(height * width);
        let mut dir_indices: Vec<u8> = (0..4).collect();

        // entry on top left
        path[0][0] |= UP;
        visited.push(Point { x: 0, y: 0 });

        while !visited.is_empty() {
            // we always start from the last cell
            let cell = &visited[visited.len() - 1];

            // shuffle directions
            dir_indices.shuffle(&mut thread_rng());

            let mut found = false;
            for &dir_index in dir_indices.iter() {
                let dir = &DIRECTIONS[dir_index as usize];

                let nx = cell.x + dir.delta.x;
                let ny = cell.y + dir.delta.y;

                // check whether new cell is unvisited
                if nx >= 0
                    && ny >= 0
                    && nx < width as i32
                    && ny < height as i32
                    && path[ny as usize][nx as usize] == 0
                {
                    // carve passage
                    path[cell.y as usize][cell.x as usize] |= dir.passage;
                    path[ny as usize][nx as usize] |= dir.opposite;
                    // continue with new cell
                    visited.push(Point { x: nx, y: ny });
                    found = true;
                    break;
                }
            }

            if !found {
                visited.pop();
            }
        }

        // exit bottom right
        path[height - 1][width - 1] |= DOWN;

        Maze {
            path,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let maze = Maze::growing_tree(10, 10);
        maze.render_ascii();
        println!("{:?}", maze);
        assert!(false);
    }
}
