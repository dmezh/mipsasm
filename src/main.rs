use anyhow::Result;

use clap::Parser as ClapParser;
use pest::Parser as PestParser;

#[derive(pest_derive::Parser)]
#[grammar = "mipsasm.pest"]
struct MipsParser;

#[derive(clap::Parser)]
struct Args {
    input: String
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.input)?;

    let pairs = MipsParser::parse(Rule::program, &input)?;

    println!("{:#?}", pairs);

    Ok(())
}
