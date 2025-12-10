mod bytecode;
mod interpreter;
mod parser;
mod jit;
mod sandbox;
mod cli;

use clap::Parser;
use cli::CinderCli;

fn main() -> anyhow::Result<()> {
    let cli = CinderCli::parse();
    cli.execute()
}

