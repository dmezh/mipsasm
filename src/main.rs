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

    let lines = MipsParser::parse(Rule::program, &input)?;

    let mut parser = MipsParser::new();

    for line in lines {
        parser.add_line(line);
    }

    let instructions = parser.resolve_instructions();
    instructions.iter().for_each(|h| println!("0x{:08X}", h));

    Ok(())
}
