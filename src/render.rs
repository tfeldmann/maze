use crate::maze;

// https://www.utf8-chartable.de/unicode-utf8-table.pl?start=9472&number=512

/*
U
  R
U R
    D
U   D
  R D
U R D
      L
U     L
  R   L
U R   L
    D L
U   D L
  R D L
U R D L
*/

const STRAIGHT: [char; 16] = [
    ' ',   //
    '╵', //  U
    '╶', //    R
    '└', //  U R
    '╷', //      D
    '│', //  U   D
    '┌', //    R D
    '├', //  U R D
    '╴', //        L
    '┘', //  U     L
    '─', //    R
    '┴', //  U R   L
    '┐', //      D L
    '┤', //  U   D L
    '┬', //    R D L
    '┼', //  U R D L
];

const STRAIGHT_WIDE: [&str; 16] = [
    "  ",     //
    "╵ ",   //  U
    "╶─", //    R
    "└─", //  U R
    "╷ ",   //      D
    "│ ",   //  U   D
    "┌─", //    R D
    "├─", //  U R D
    "╴ ",   //        L
    "┘ ",   //  U     L
    "──", //    R
    "┴─", //  U R   L
    "┐ ",   //      D L
    "┤ ",   //  U   D L
    "┬─", //    R D L
    "┼─", //  U R D L
];

const ROUND: [char; 16] = [
    ' ',   //
    '╵', // U
    '╶', //   R
    '╰', // U R
    '╷', //     D
    '│', // U   D
    '╭', //   R D
    '├', // U R D
    '╴', //       L
    '╯', // U     L
    '─', //   R   L
    '┴', // U R   L
    '╮', //     D L
    '┤', // U   D L
    '┬', //   R D L
    '┼', // U R D L
];

const ROUND_WIDE: [&str; 16] = [
    "  ",     //
    "╵ ",   // U
    "╶─", //   R
    "╰─", // U R
    "╷ ",   //     D
    "│ ",   // U   D
    "╭─", //   R D
    "├─", // U R D
    "╴ ",   //       L
    "╯ ",   // U     L
    "──", //   R   L
    "┴─", // U R   L
    "╮ ",   //     D L
    "┤ ",   // U   D L
    "┬─", //   R D L
    "┼─", // U R D L
];

// pub fn print_logic_table() {
//     for p in 0..=0b1111 {
//         print!("{:2}:", p);
//         print!(" {}", if p & maze::UP != 0 { "U" } else { " " });
//         print!(" {}", if p & maze::RIGHT != 0 { "R" } else { " " });
//         print!(" {}", if p & maze::DOWN != 0 { "D" } else { " " });
//         print!(" {}", if p & maze::LEFT != 0 { "L" } else { " " });
//         println!();
//     }
// }

pub fn path(grid: &Vec<Vec<u8>>) {
    for y in (0..grid.len()).rev() {
        for x in 0..grid[0].len() {
            print!("{}", STRAIGHT_WIDE[grid[y][x] as usize]);
        }
        println!();
    }
}

pub fn unicode_thick(grid: &Vec<Vec<u8>>) -> String {
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
