use crate::maze;

// const BORDER: [char; 11] = ['┴', '├', '┬', '┤', '┼', '─', '│', '┌', '┐', '└', '┘'];
/*
─	━	│	┃	┄	┅	┆	┇	┈	┉	┊	┋	┌	┍	┎	┏
U+251x	┐	┑	┒	┓	└	┕	┖	┗	┘	┙	┚	┛	├	┝	┞	┟
U+252x	┠	┡	┢	┣	┤	┥	┦	┧	┨	┩	┪	┫	┬	┭	┮	┯
U+253x	┰	┱	┲	┳	┴	┵	┶	┷	┸	┹	┺	┻	┼	┽	┾	┿
U+254x	╀	╁	╂	╃	╄	╅	╆	╇	╈	╉	╊	╋	╌	╍	╎	╏
*/

pub fn unicode_thick(grid: Vec<Vec<u8>>) -> String {
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
