use maze_lib::unicode::{THEME_HEAVY, THEME_ROUND, THEME_STRAIGHT};
use maze_lib::Maze;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn maze(width: usize, height: usize, theme: usize) -> String {
    let t = match theme {
        1 => THEME_STRAIGHT,
        2 => THEME_ROUND,
        3 => THEME_HEAVY,
        _ => THEME_STRAIGHT,
    };
    let maze = Maze::growing_tree(width, height);
    maze.unicode_walls(&t, true)
}
