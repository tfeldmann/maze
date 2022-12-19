use clap::{Parser, ValueEnum};
use std::time::Instant;

use maze_lib::unicode::{THEME_HEAVY, THEME_ROUND, THEME_STRAIGHT};
use maze_lib::Maze;

#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Cli {
    /// The theme
    #[arg(short, long, value_enum, default_value_t = Theme::Walls)]
    theme: Theme,
    /// Maze width in blocks
    #[clap(default_value_t = 20)]
    width: usize,
    /// Maze height in blocks
    #[clap(default_value_t = 10)]
    height: usize,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Theme {
    Walls,
    WallsHeavy,
    WallsRound,
    WallsNarrow,
    WallsHeavyNarrow,
    WallsRoundNarrow,
    Path,
    PathHeavy,
    PathRound,
    PathNarrow,
    PathHeavyNarrow,
    PathRoundNarrow,
}

fn main() {
    let args = Cli::parse();

    let now = Instant::now();
    let maze = Maze::growing_tree(args.width, args.height);
    let elapsed_time = now.elapsed();

    let result = match args.theme {
        Theme::Walls => maze.unicode_walls(&THEME_STRAIGHT, true),
        Theme::WallsHeavy => maze.unicode_walls(&THEME_HEAVY, true),
        Theme::WallsRound => maze.unicode_walls(&THEME_ROUND, true),
        Theme::WallsNarrow => maze.unicode_walls(&THEME_STRAIGHT, false),
        Theme::WallsHeavyNarrow => maze.unicode_walls(&THEME_HEAVY, false),
        Theme::WallsRoundNarrow => maze.unicode_walls(&THEME_ROUND, false),

        Theme::Path => maze.unicode_path(&THEME_STRAIGHT, true),
        Theme::PathHeavy => maze.unicode_path(&THEME_HEAVY, true),
        Theme::PathRound => maze.unicode_path(&THEME_ROUND, true),
        Theme::PathNarrow => maze.unicode_path(&THEME_STRAIGHT, false),
        Theme::PathHeavyNarrow => maze.unicode_path(&THEME_HEAVY, false),
        Theme::PathRoundNarrow => maze.unicode_path(&THEME_ROUND, false),
    };
    print!("{}", result);

    if args.verbose {
        println!("Maze generated in {} ms.", elapsed_time.as_millis());
    }
}
