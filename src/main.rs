use std::env;
use std::time::Instant;
mod maze;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let now = Instant::now();
    let m = maze::maze(10, 10);
    let elapsed_time = now.elapsed();

    maze::print_grid(m);
    println!("Maze generated in {} ms.", elapsed_time.as_millis());
}

// const BORDER: [char; 11] = ['┴', '├', '┬', '┤', '┼', '─', '│', '┌', '┐', '└', '┘'];
/*
─	━	│	┃	┄	┅	┆	┇	┈	┉	┊	┋	┌	┍	┎	┏
U+251x	┐	┑	┒	┓	└	┕	┖	┗	┘	┙	┚	┛	├	┝	┞	┟
U+252x	┠	┡	┢	┣	┤	┥	┦	┧	┨	┩	┪	┫	┬	┭	┮	┯
U+253x	┰	┱	┲	┳	┴	┵	┶	┷	┸	┹	┺	┻	┼	┽	┾	┿
U+254x	╀	╁	╂	╃	╄	╅	╆	╇	╈	╉	╊	╋	╌	╍	╎	╏
*/
