use std::time::Instant;

use clap::Parser;

mod loader;

type AdventOfCodeSolveFunctions = &'static [(&'static str, fn(String) -> String, fn(String) -> String)];

static ADVENTOFCODE_SOLVE_FUNCTIONS: AdventOfCodeSolveFunctions = &[];

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

    let session = loader::Session{ session_token: args.session_token };
    for function in ADVENTOFCODE_SOLVE_FUNCTIONS.iter() {
        if function.0 == args.puzzle {
            println!("Challenge {} ({}/2)", function.0, args.puzzle_part);
            let input = session.get_puzzle_input(function.0)
                .expect("Error while getting puzzle input for");
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
