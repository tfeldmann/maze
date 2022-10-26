use crate::maze;

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

pub fn draw_path(grid: &Vec<Vec<u8>>, theme: PathTheme, wide: bool) -> String {
    let height = grid.len();
    let width = grid[0].len();

    let mut result = String::with_capacity(height * width * (wide as usize + 1));
    for y in 0..height {
        for x in 0..width {
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

pub fn _walls2(path: &Vec<Vec<u8>>) -> String {
    // we create walls, which are a path *around* the given path
    let height = path.len() + 1;
    let width = path[0].len() + 1;

    let mut walls: Vec<Vec<u8>> = vec![vec![0; width as usize]; height as usize];
    // each cell in walls indicates, which neighbor cells dont have passages in the
    // given direction.

    for y in 0..height {
        for x in 0..width {
            /*
            0,0: -1,-1  0,-1  -1,0  0,0
            1,0:  0,-1  1,-1  0,0   1,0
            2,0:  0,0
            3,0:  0,0

            Wall 1,1:
                Hoch?
                    0,0 rechts
                Rechts?
                    1,0 unten oder 1,1 oben
            */
            print!("{:2} {:2} |", x, y);
        }
        println!();
    }

    let mut result = String::new();
    result = draw_path(path, THEME_STRAIGHT, true);
    result
}

pub fn walls(grid: &Vec<Vec<u8>>) -> String {
    let mut result = String::with_capacity(10);
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
