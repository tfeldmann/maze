use clap::{Parser, ValueEnum};
use std::time::Instant;

mod maze;
mod render;

#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Cli {
    /// The theme
    #[arg(short, long, value_enum, default_value_t = Theme::Walls)]
    theme: Theme,
    /// Maze width in blocks
    #[clap(default_value_t = 20, value_parser=clap::value_parser!(i32).range(1..))]
    width: i32,
    /// Maze height in blocks
    #[clap(default_value_t = 10, value_parser=clap::value_parser!(i32).range(1..))]
    height: i32,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Theme {
    Path,
    PathHeavy,
    PathRound,
    PathNarrow,
    PathHeavyNarrow,
    PathRoundNarrow,
    Walls,
}

fn main() {
    let args = Cli::parse();

    let now = Instant::now();
    let m = maze::maze(args.width, args.height);
    let elapsed_time = now.elapsed();

    let result = match args.theme {
        Theme::Path => render::draw_path(&m, render::THEME_STRAIGHT, true),
        Theme::PathHeavy => render::draw_path(&m, render::THEME_HEAVY, true),
        Theme::PathRound => render::draw_path(&m, render::THEME_ROUND, true),
        Theme::PathNarrow => render::draw_path(&m, render::THEME_STRAIGHT, false),
        Theme::PathHeavyNarrow => render::draw_path(&m, render::THEME_HEAVY, false),
        Theme::PathRoundNarrow => render::draw_path(&m, render::THEME_ROUND, false),
        Theme::Walls => render::walls(&m),
    };
    print!("{}", result);

    if args.verbose {
        println!("Maze generated in {} ms.", elapsed_time.as_millis());
    }
}
