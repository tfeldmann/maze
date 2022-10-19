use clap::{Parser, ValueEnum};
use std::time::Instant;

mod maze;
mod render;

#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Cli {
    /// The theme
    #[arg(short='t', long="theme", value_enum, default_value_t = Theme::Path)]
    theme: Theme,
    /// Maze width in blocks
    #[clap(default_value_t = 20, value_parser=clap::value_parser!(i32).range(1..))]
    width: i32,
    /// Maze height in blocks
    #[clap(default_value_t = 10, value_parser=clap::value_parser!(i32).range(1..))]
    height: i32,
    // #[clap(long, short)]
    // verbose: Option<bool>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Theme {
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
    let m = maze::maze(args.width, args.height);
    let elapsed_time = now.elapsed();

    let theme_result = match args.theme {
        Theme::Path => render::path(&m, render::PATH_STRAIGHT, true),
        Theme::PathHeavy => render::path(&m, render::PATH_HEAVY, true),
        Theme::PathRound => render::path(&m, render::PATH_ROUND, true),
        Theme::PathNarrow => render::path(&m, render::PATH_STRAIGHT, false),
        Theme::PathHeavyNarrow => render::path(&m, render::PATH_HEAVY, false),
        Theme::PathRoundNarrow => render::path(&m, render::PATH_ROUND, false),
    };
    let result = render::walls(&m);
    println!("{}", result);
    println!("{}", theme_result);
    println!("Maze generated in {} ms.", elapsed_time.as_millis());
}
