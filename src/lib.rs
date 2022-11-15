#![allow(dead_code)] // unfortunate but modular-bitfield keeps throwing warnings

use modular_bitfield::{bitfield, specifiers::*};
pub use pest::{
    iterators::{Pair, Pairs},
    Parser as PestParser,
};

#[derive(pest_derive::Parser)]
#[grammar = "mipsasm.pest"]
pub struct MipsParser;

#[bitfield(bits = 32)]
pub struct RTypeInstruction {
    funct: B6,
    shamt: B5,
    rd: B5,
    rt: B5,
    rs: B5,
    op: B6, // todo could get rid of op; it's always 0
}

#[bitfield(bits = 32)]
pub struct ITypeInstruction {
    imm: B16,
    rt: B5,
    rs: B5,
    op: B6,
}

impl MipsParser {
    pub fn encode_imm_instr(args: Pairs<Rule>) -> u32 {
        let args = Self::args_to_rule_str_pairs(args);

        let op = match args[0].0 {
            Rule::op_addi => 8,
            _ => panic!("Unexpected rule"),
        };

        let rt: u8 = args[1].1.parse().unwrap();
        let rs: u8 = args[2].1.parse().unwrap();
        let imm: i16 = args[3].1.parse().unwrap();

        let instr = ITypeInstruction::new()
            .with_op(op)
            .with_rs(rs)
            .with_rt(rt)
            .with_imm(imm as u16);

        u32::from_le_bytes(instr.into_bytes())
    }

    pub fn encode_mem_instr(args: Pairs<Rule>) -> u32 {
        let args = Self::args_to_rule_str_pairs(args);

        let op = match args[0].0 {
            Rule::op_sw => 43,
            Rule::op_lw => 35,
            _ => panic!("Unexpected rule")
        };

        let rt: u8 = args[1].1.parse().unwrap();
        let imm: i16 = args[2].1.parse().unwrap();
        let rs: u8 = args[3].1.parse().unwrap();

        let instr = ITypeInstruction::new()
            .with_op(op)
            .with_rs(rs)
            .with_rt(rt)
            .with_imm(imm as u16);

        u32::from_le_bytes(instr.into_bytes())
    }

    pub fn encode_arith_instr(args: Pairs<Rule>) -> u32 {
        let args = Self::args_to_rule_str_pairs(args);

        let funct = match args[0].0 {
            Rule::op_add => 32,
            Rule::op_or => 37,
            Rule::op_and => 36,
            Rule::op_slt => 42,
            Rule::op_sub => 34,
            _ => panic!("Unexpected rule"),
        };

        let rd: u8 = args[1].1.parse().unwrap();
        let rs: u8 = args[2].1.parse().unwrap();
        let rt: u8 = args[3].1.parse().unwrap();

        let instr = RTypeInstruction::new()
            .with_op(0) // all R-type have op 0
            .with_rs(rs)
            .with_rt(rt)
            .with_rd(rd)
            .with_shamt(0)
            .with_funct(funct);

        u32::from_le_bytes(instr.into_bytes())
    }

    pub fn parse_instruction(instruction: Pair<Rule>) -> u32 {
        let op = instruction.as_rule();
        let args = instruction.into_inner();

        println!("{:#?}", op);

        match op {
            Rule::imm_instruction => MipsParser::encode_imm_instr(args),
            Rule::reg_3_arith_instruction => MipsParser::encode_arith_instr(args),
            Rule::mem_instruction => MipsParser::encode_mem_instr(args),
            _ => panic!("Error parsing: expected instruction"),
        }
    }

    fn args_to_rule_str_pairs(args: Pairs<Rule>) -> Vec<(Rule, String)> {
        args.map(|p| (p.as_rule(), p.as_span().as_str().into()))
            .collect()
    }
}
