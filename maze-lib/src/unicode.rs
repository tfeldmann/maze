/*
UTF-8 table:

    https://www.vidarholen.net/cgi-bin/labyrinth?w=39&h=24
    https://www.utf8-chartable.de/unicode-utf8-table.pl?start=9472&number=512

Theme definition:

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

pub struct Theme {
    chars: [char; 16],
    widen_char: char,
}

impl Theme {
    fn for_directions(&self, directions: u8, wide: bool) -> String {
        let theme_char = self.chars[directions as usize];
        if wide {
            let wide_char = if directions & RIGHT != 0 {
                self.widen_char
            } else {
                ' '
            };
            format!("{}{}", theme_char, wide_char)
        } else {
            theme_char.to_string()
        }
    }
}

pub const THEME_STRAIGHT: Theme = Theme {
    chars: [
        ' ', '╵', '╶', '└', '╷', '│', '┌', '├', '╴', '┘', '─', '┴', '┐', '┤', '┬', '┼',
    ],
    widen_char: '─',
};

pub const THEME_HEAVY: Theme = Theme {
    chars: [
        ' ', '╹', '╺', '┗', '╻', '┃', '┏', '┣', '╸', '┛', '━', '┻', '┓', '┫', '┳', '╋',
    ],
    widen_char: '━',
};

pub const THEME_ROUND: Theme = Theme {
    chars: [
        ' ', '╵', '╶', '╰', '╷', '│', '╭', '├', '╴', '╯', '─', '┴', '╮', '┤', '┬', '┼',
    ],
    widen_char: '─',
};

impl Maze {
    pub fn unicode_path(&self, theme: &Theme, wide: bool) -> String {
        let mut result =
            String::with_capacity(self.height * (self.width + 1) * (wide as usize + 1));
        for y in 0..self.height {
            for x in 0..self.width {
                let dirs = self.path[y][x];
                let is_wide = wide && x < self.width - 1;
                result.push_str(&theme.for_directions(dirs, is_wide));
            }
            result.push('\n');
        }
        result
    }

    fn dirs(&self, x: i32, y: i32) -> u8 {
        let (x_max, y_max) = (self.width as i32, self.height as i32);
        if x >= 0 && x < x_max && y >= 0 && y < y_max {
            self.path[y as usize][x as usize]
        } else {
            let up = x == -1 || x == x_max;
            let left = y == -1 || y == y_max;
            up as u8 * UP + left as u8 * LEFT
        }
    }

    fn wall_dirs(&self, x: i32, y: i32) -> u8 {
        let u = self.dirs(x, y - 1) & LEFT == 0;
        let r = self.dirs(x, y) & UP == 0;
        let d = self.dirs(x, y) & LEFT == 0;
        let l = self.dirs(x - 1, y) & UP == 0;
        u as u8 * UP + r as u8 * RIGHT + d as u8 * DOWN + l as u8 * LEFT
    }

    pub fn unicode_walls(&self, theme: &Theme, wide: bool) -> String {
        let mut result = String::new();
        for y in 0..=self.height {
            for x in 0..=self.width {
                let is_wide = wide && x < self.width;
                result.push_str(&theme.for_directions(self.wall_dirs(x as i32, y as i32), is_wide));
            }
            result.push('\n');
        }
        result
    }
}
