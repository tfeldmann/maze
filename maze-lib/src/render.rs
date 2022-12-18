/*
UTF-8 table:

    https://www.utf8-chartable.de/unicode-utf8-table.pl?start=9472&number=512

Passages for grid values:

     0:
     1: U
     2:   R
     3: U R
     4:     D
     5: U   D
     6:   R D
     7: U R D
     8:       L
     9: U     L
    10:   R   L
    11: U R   L
    12:     D L
    13: U   D L
    14:   R D L
    15: U R D L
*/
use crate::{Maze, DOWN, LEFT, RIGHT, UP};

pub struct PathTheme {
    chars: [char; 16],
    widen_char: char,
}

pub const THEME_STRAIGHT: PathTheme = PathTheme {
    chars: [
        ' ', '╵', '╶', '└', '╷', '│', '┌', '├', '╴', '┘', '─', '┴', '┐', '┤', '┬', '┼',
    ],
    widen_char: '─',
};

pub const THEME_HEAVY: PathTheme = PathTheme {
    chars: [
        ' ', '╹', '╺', '┗', '╻', '┃', '┏', '┣', '╸', '┛', '━', '┻', '┓', '┫', '┳', '╋',
    ],
    widen_char: '━',
};

pub const THEME_ROUND: PathTheme = PathTheme {
    chars: [
        ' ', '╵', '╶', '╰', '╷', '│', '╭', '├', '╴', '╯', '─', '┴', '╮', '┤', '┬', '┼',
    ],
    widen_char: '─',
};

impl Maze {
    pub fn unicode_path(&self, theme: PathTheme, wide: bool) -> String {
        let mut result = String::with_capacity(self.height * self.width * (wide as usize + 1));
        for y in 0..self.height {
            for x in 0..self.width {
                let passages = self.path[y][x];
                result.push(theme.chars[passages as usize]);
                if wide && x != self.width - 1 {
                    if passages & RIGHT != 0 {
                        result.push(theme.widen_char);
                    } else {
                        result.push(' ');
                    }
                }
            }
            result.push('\n');
        }
        result
    }

    pub fn unicode_walls(&self, theme: PathTheme, wide: bool) -> String {
        "unicode walls".to_string()
    }
}

pub fn walls(path: &Vec<Vec<u8>>) -> String {
    // let mut path: Vec<Vec<u8>> = vec![vec![0; 3]; 3];
    // path[0][0] = UP | DOWN;
    // path[0][1] = DOWN | RIGHT;
    // path[0][2] = LEFT | DOWN;
    // path[1][0] = UP | RIGHT;
    // path[1][1] = LEFT | UP | DOWN;
    // path[1][2] = UP;
    // path[2][0] = RIGHT;
    // path[2][1] = LEFT | RIGHT | UP;
    // path[2][2] = LEFT | DOWN;

    // we create walls, which are a path *around* the given path
    let height = path.len();
    let width = path[0].len();

    let mut edges: Vec<Vec<u8>> = vec![vec![0; width as usize + 1]; height as usize + 1];
    // each cell in walls indicates, which neighbor cells dont have passages in the
    // given direction.

    for y in 0..height {
        for x in 0..width + 1 {
            let up = if y > 0 && x < width {
                path[y - 1][x] & LEFT == 0
            } else {
                x == width
            };
            let right = if x < width {
                path[y][x] & UP == 0
            } else {
                false
            };
            let down = if x < width {
                path[y][x] & LEFT == 0
            } else {
                true
            };
            let left = if x > 0 {
                path[y][x - 1] & UP == 0
            } else {
                false
            };

            let mut cell = 0;
            if up {
                cell |= UP;
            }
            if down {
                cell |= DOWN;
            }
            if right {
                cell |= RIGHT;
            }
            if left {
                cell |= LEFT;
            }

            edges[y][x] = cell;
        }
        println!();
    }
    todo!()
}
