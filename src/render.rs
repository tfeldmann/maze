use crate::maze;

// https://www.utf8-chartable.de/unicode-utf8-table.pl?start=9472&number=512

/*
Grid passages:
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

pub struct PathTheme {
    chars: [char; 16],
    widen_char: char,
}

pub const PATH_STRAIGHT: PathTheme = PathTheme {
    chars: [
        ' ', '╵', '╶', '└', '╷', '│', '┌', '├', '╴', '┘', '─', '┴', '┐', '┤', '┬', '┼',
    ],
    widen_char: '─',
};

pub const PATH_HEAVY: PathTheme = PathTheme {
    chars: [
        ' ', '╹', '╺', '┗', '╻', '┃', '┏', '┣', '╸', '┛', '━', '┻', '┓', '┫', '┳', '╋',
    ],
    widen_char: '━',
};

pub const PATH_ROUND: PathTheme = PathTheme {
    chars: [
        ' ', '╵', '╶', '╰', '╷', '│', '╭', '├', '╴', '╯', '─', '┴', '╮', '┤', '┬', '┼',
    ],
    widen_char: '─',
};

pub fn path(grid: &Vec<Vec<u8>>, theme: PathTheme, wide: bool) -> String {
    let mut result = String::new();
    let width = grid[0].len();
    for y in (0..grid.len()).rev() {
        for x in 0..grid[0].len() {
            let passages = grid[y][x];
            result.push(theme.chars[passages as usize]);
            if wide && x != width - 1 {
                if passages & maze::RIGHT != 0 {
                    result.push(theme.widen_char);
                } else {
                    result.push(' ');
                }
            }
        }
        result.push('\n');
    }
    return result;
}

pub fn walls(grid: &Vec<Vec<u8>>) -> String {
    let mut result = String::new();
    for row in grid.iter() {
        for cell in row.iter() {
            if (cell & maze::DOWN) != 0 {
                result.push_str("██  ");
            } else {
                result.push_str("████");
            }
        }
        if (row[row.len() - 1] & maze::RIGHT) != 0 {
            result.push_str("  \n");
        } else {
            result.push_str("██\n");
        }

        for cell in row.iter() {
            if (cell & maze::LEFT) != 0 {
                result.push_str("    ");
            } else {
                result.push_str("██  ");
            }
        }
        result.push_str("██\n");
    }
    for _ in 0..grid[0].len() {
        result.push_str("████");
    }
    result.push_str("██\n");
    return result;
}
