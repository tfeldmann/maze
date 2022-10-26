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

pub fn walls(path: &Vec<Vec<u8>>) -> String {
    // let mut path: Vec<Vec<u8>> = vec![vec![0; 3]; 3];
    // path[0][0] = maze::UP | maze::DOWN;
    // path[0][1] = maze::DOWN | maze::RIGHT;
    // path[0][2] = maze::LEFT | maze::DOWN;
    // path[1][0] = maze::UP | maze::RIGHT;
    // path[1][1] = maze::LEFT | maze::UP | maze::DOWN;
    // path[1][2] = maze::UP;
    // path[2][0] = maze::RIGHT;
    // path[2][1] = maze::LEFT | maze::RIGHT | maze::UP;
    // path[2][2] = maze::LEFT | maze::DOWN;

    // we create walls, which are a path *around* the given path
    let height = path.len();
    let width = path[0].len();

    let mut edges: Vec<Vec<u8>> = vec![vec![0; width as usize + 1]; height as usize + 1];
    // each cell in walls indicates, which neighbor cells dont have passages in the
    // given direction.

    for y in 0..height + 1 {
        for x in 0..width + 1 {
            let mut cell = 0;

            let no_path_above = x < width && y > 0 && path[y - 1][x] & maze::LEFT != 0;
            let no_path_below = x < width && y < height && path[y][x] & maze::LEFT != 0;
            let no_path_right = x < width && y < height && path[y][x] & maze::UP != 0;
            let no_path_left = x > 0 && y < height && path[y][x - 1] & maze::UP != 0;

            if !no_path_above {
                cell |= maze::UP;
            }
            if !no_path_below {
                cell |= maze::DOWN;
            }
            if !no_path_right {
                cell |= maze::RIGHT;
            }
            if !no_path_left {
                cell |= maze::LEFT;
            }

            edges[y][x] = cell;
        }
        println!();
    }

    println!("{}", draw_path(&path, THEME_STRAIGHT, true));
    let result = draw_path(&edges, THEME_STRAIGHT, true);
    result
}
