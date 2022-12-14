use std::{time::Instant, path::PathBuf, fs::create_dir};

use clap::Parser;

mod loader;

mod one;
mod second;
mod third;
mod fourth;
mod fifth;
mod sixth;
mod seventh;

type AdventOfCodeSolveFunctions = &'static [(&'static str, fn(String) -> String, fn(String) -> String)];

static ADVENTOFCODE_SOLVE_FUNCTIONS: AdventOfCodeSolveFunctions = &[
    ("1", one::solve_first, one::solve_second),
    ("2", second::solve_first, second::solve_second),
    ("3", third::solve_first, third::solve_second),
    ("4", fourth::solve_first, fourth::solve_second),
    ("5", fifth::solve_first, fifth::solve_second),
    ("6", sixth::solve_first, sixth::solve_second),
    ("7", seventh::solve_first, seventh::solve_second)
];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    session_token: String,
    #[arg(short, long, default_value_t=1)]
    puzzle_part: u8,
    #[arg(value_name="puzzle")]
    puzzle: String,
}

fn main() {
    let args = Args::parse();

    let cache_path = PathBuf::from(".cache");
    if !cache_path.exists() {
        create_dir(cache_path.clone()).unwrap();
    }

    let session = loader::Session{ session_token: args.session_token };
    let cache = loader::Cache::new(cache_path, session);
    for function in ADVENTOFCODE_SOLVE_FUNCTIONS.iter() {
        if function.0 == args.puzzle {
            println!("Challenge {} ({}/2)", function.0, args.puzzle_part);
            let input = cache.get_puzzle_input(function.0);
            let now = Instant::now();
            let output = match args.puzzle_part {
                1 => function.1,
                2 => function.2,
                _ => function.1
            }(input);
            println!("Took {:.2?}ms", now.elapsed().as_millis());
            println!("Answer: {output}");
        }
    }
}
