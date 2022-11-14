use anyhow::Result;

use clap::Parser as ClapParser;
use pest::{iterators::Pairs, Parser as PestParser};

#[derive(pest_derive::Parser)]
#[grammar = "mipsasm.pest"]
struct MipsParser;

#[derive(clap::Parser)]
struct Args {
    input: String,
}

impl MipsParser {
    pub fn encode_imm_instr(args: Pairs<Rule>) -> String {
        "".into()
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.input)?;

    let lines = MipsParser::parse(Rule::program, &input)?;
    let mut output = String::new();

    for line in lines {
        for instruction in line.into_inner() {
            let op = instruction.as_rule();
            let args = instruction.into_inner();

            println!("{:#?}", op);

            let hex = match op {
                Rule::imm_instruction => MipsParser::encode_imm_instr(args),
                _ => panic!("Error parsing: expected instruction"),
            };

            output = [output, hex].join("\n");
        }
    }

    println!("OUTPUT:\n{}", output);

    Ok(())
}
