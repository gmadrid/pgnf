use argh::FromArgs;
use std::fs::read_to_string;
use thiserror::Error;

#[derive(FromArgs)]
/// DO NOT SUBMIT without putting something here. TODO
struct Args {
    /// DO NOT SUBMIT without filling this in TODO
    #[argh(positional)]
    pgn_files: Vec<String>,
}

#[derive(Debug, Error)]
enum Err {
    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("{0}")]
    PgnError(#[from] pgntool::PgnError),
}

type Result<T> = std::result::Result<T, Err>;

fn process_stdin() -> Result<()> {
    println!("NO ARGS");
    Ok(())
}

fn process_pgn_files(args: Args) -> Result<()> {
    for file in args.pgn_files {
        let pgn_string = read_to_string(file)?;
        let database = pgntool::parse_pgn(pgn_string)?;
        dbg!(database);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    if args.pgn_files.is_empty() {
        process_stdin()
    } else {
        process_pgn_files(args)
    }
}
