use anyhow::Result;
use clap::Parser as ClapParser;
use modular_bitfield::{bitfield, specifiers::*};
use pest::{iterators::Pairs, Parser as PestParser, Span};

#[derive(pest_derive::Parser)]
#[grammar = "mipsasm.pest"]
struct MipsParser;

#[derive(clap::Parser)]
struct Args {
    input: String,
}

#[bitfield(bits = 32)]
struct instr_rtype {
    funct: B6,
    shamt: B5,
    rd: B5,
    rt: B5,
    rs: B5,
    op: B6, // todo could get rid of op; it's always 0
}

impl MipsParser {
    pub fn encode_imm_instr(args: Pairs<Rule>) -> u32 {
        todo!()
    }

    pub fn encode_arith_instr(args: Pairs<Rule>) -> u32 {
        let args: Vec<(Rule, String)> = args
            .map(|p| (p.as_rule(), p.as_span().as_str().into()))
            .collect();

        let funct = match args[0].0 {
            Rule::op_add => 32,
            Rule::op_or  => 37,
            _ => panic!(),
        };

        let rd: u8 = args[1].1.parse().unwrap();
        let rs: u8 = args[2].1.parse().unwrap();
        let rt: u8 = args[3].1.parse().unwrap();

        let instr = instr_rtype::new()
            .with_op(0) // all R-type have op 0
            .with_rs(rs)
            .with_rt(rt)
            .with_rd(rd)
            .with_shamt(0)
            .with_funct(funct);

        u32::from_le_bytes(instr.into_bytes())
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
                Rule::reg_3_arith_instruction => MipsParser::encode_arith_instr(args),
                _ => panic!("Error parsing: expected instruction"),
            };

            println!("0b{:b}", hex);
            println!("0x{:08X}", hex);

            output = [output, hex.to_string()].join("\n");
        }
    }

    println!("OUTPUT:\n{}", output);

    Ok(())
}
