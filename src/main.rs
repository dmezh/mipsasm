use anyhow::Result;
use clap::Parser as ClapParser;

use mipsasm::*;

#[derive(clap::Parser)]
struct Args {
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.input)?;

    let instructions = MipsParser::parse_and_resolve_entire(&input);

    instructions.iter().for_each(|h| println!("0x{:08x}", h));

    Ok(())
}
