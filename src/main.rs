use std::time::Instant;

use clap::Parser;
mod maze;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(name = "maze")]
#[clap(author, version, about, long_about=None)]
struct Cli {
    #[clap(default_value_t = 10, value_parser=clap::value_parser!(i64).range(1..))]
    width: i64,
    #[clap(default_value_t = 10, value_parser=clap::value_parser!(i64).range(1..))]
    height: i64,
}

fn main() {
    let args = Cli::parse();

    let now = Instant::now();
    let m = maze::maze(args.width, args.height);
    let elapsed_time = now.elapsed();

    maze::print_grid(m);
    println!("Maze generated in {} ms.", elapsed_time.as_millis());
}
