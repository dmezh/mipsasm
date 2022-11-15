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
    let mut output = String::new();

    for line in lines {
        for instruction in line.into_inner() {
            let hex = MipsParser::parse_instruction(instruction);

            println!("0b{:b}", hex);
            println!("0x{:08X}", hex);
            output = [output, hex.to_string()].join("\n");
        }
    }

    println!("OUTPUT:\n{}", output);

    Ok(())
}
