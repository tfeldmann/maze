use clap::Parser;
use std::time::Instant;

mod maze;
mod render;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Cli {
    #[clap(default_value_t = 20, value_parser=clap::value_parser!(i32).range(1..))]
    width: i32,
    #[clap(default_value_t = 10, value_parser=clap::value_parser!(i32).range(1..))]
    height: i32,
    // #[clap(long, short)]
    // verbose: Option<bool>,
}

fn main() {
    let args = Cli::parse();

    let now = Instant::now();
    let m = maze::maze(args.width, args.height);
    let elapsed_time = now.elapsed();

    let path_result = render::path(&m, render::PATH_HEAVY, true);
    let result = render::walls(&m);
    println!("{}", result);
    println!("{}", path_result);
    println!("Maze generated in {} ms.", elapsed_time.as_millis());
}
