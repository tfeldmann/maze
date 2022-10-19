use clap::Parser;
use std::time::Instant;

mod maze;
mod render;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Cli {
    #[clap(default_value_t = 20, value_parser=clap::value_parser!(i64).range(1..))]
    width: i64,
    #[clap(default_value_t = 10, value_parser=clap::value_parser!(i64).range(1..))]
    height: i64,
    // #[clap(long, short)]
    // verbose: Option<bool>,
}

fn main() {
    let args = Cli::parse();

    let now = Instant::now();
    let m = maze::maze(args.width, args.height);
    let elapsed_time = now.elapsed();

    render::path(&m);
    let result = render::unicode_thick(&m);
    println!("{}", result);
    println!("Maze generated in {} ms.", elapsed_time.as_millis());
}
