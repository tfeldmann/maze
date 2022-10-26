// Algorithm based on:
// https://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm

extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

// TODO: Use bitflags crate for type safety
pub const UP: u8 = 0b0001;
pub const RIGHT: u8 = 0b0010;
pub const DOWN: u8 = 0b0100;
pub const LEFT: u8 = 0b1000;

struct Point {
    x: i32,
    y: i32,
}

struct Direction {
    passage: u8,
    opposite: u8,
    delta: Point,
}

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

pub fn maze(width: i32, height: i32) -> Vec<Vec<u8>> {
    let mut path: Vec<Vec<u8>> = vec![vec![0; width as usize]; height as usize];
    let mut visited: Vec<Point> = Vec::with_capacity(height as usize * width as usize);
    let mut dir_indices: Vec<u8> = (0..4).collect();

    // entry on top left
    visited.push(Point {
        x: 0,
        y: height - 1,
    });
    path[(height - 1) as usize][0] |= UP;

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
                && nx < width
                && ny < height
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
    path[0][width as usize - 1] |= DOWN;

    return path;
}
