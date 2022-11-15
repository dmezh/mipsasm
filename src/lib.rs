#![allow(dead_code)] // unfortunate but modular-bitfield keeps throwing warnings

use std::collections::HashMap;

use modular_bitfield::{bitfield, specifiers::*};
pub use pest::{
    iterators::{Pair, Pairs},
    Parser as PestParser,
};

pub enum PotentialInstruction {
    Finished(u32),
    NeedsLabelResolution(TypedInstruction, String),
}

#[derive(pest_derive::Parser)]
#[grammar = "mipsasm.pest"]
pub struct MipsParser {
    pub labels: HashMap<String, u32>,
    pub current_addr: u32,
    pub instructions: Vec<PotentialInstruction>,
}

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

#[bitfield(bits = 32)]
pub struct JTypeInstruction {
    addr: B26,
    op: B6,
}

pub enum TypedInstruction {
    JType(JTypeInstruction),
    IType(ITypeInstruction, u32),
    RType(RTypeInstruction),
}

impl Default for MipsParser {
    fn default() -> Self {
        Self::new()
    }
}

impl MipsParser {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            current_addr: 0,
            instructions: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: Pair<Rule>) {
        for element in line.into_inner() {
            match element.as_rule() {
                Rule::label => self.add_label(element.as_str()),
                _ => {
                    self.add_instruction(element);
                    return;
                }
            };
        }

        panic!("Expected instruction in line")
    }

    pub fn resolve_instructions(self) -> Vec<u32> {
        let mut ret: Vec<u32> = Vec::new();

        for i in self.instructions {
            let (instr, label) = match i {
                PotentialInstruction::Finished(u) => {
                    ret.push(u);
                    continue;
                }
                PotentialInstruction::NeedsLabelResolution(instr, label) => (instr, label),
            };

            if let Some(a) = self.labels.get(&label) {
                let instr = match instr {
                    TypedInstruction::JType(j) => {
                        let j = j.with_addr(*a as u32 / 4);
                        u32::from_le_bytes(j.into_bytes())
                    }
                    TypedInstruction::IType(i, this_addr) => {
                        let i = i.with_imm(((*a as u32 - this_addr - 1) / 4) as u16);
                        u32::from_le_bytes(i.into_bytes())
                    }
                    _ => panic!("Unexpected instruction type needing label resolution"),
                };

                ret.push(instr);
            } else {
                panic!("Tried to use label `{label}` and it was never defined")
            }
        }

        ret
    }

    pub fn parse_instruction(&self, instruction: Pair<Rule>) -> PotentialInstruction {
        let op = instruction.as_rule();
        let args = instruction.into_inner();

        println!("{:#?}", op);

        match op {
            Rule::imm_instruction => PotentialInstruction::Finished(Self::encode_imm_instr(args)),
            Rule::reg_3_arith_instruction => {
                PotentialInstruction::Finished(Self::encode_arith_instr(args))
            }
            Rule::mem_instruction => PotentialInstruction::Finished(Self::encode_mem_instr(args)),
            Rule::br_instruction => self.encode_br_instr(args),
            Rule::j_instruction => Self::encode_j_instr(args),
            p => panic!("Error parsing: expected an instruction, got `{:#?}`", p),
        }
    }

    fn encode_imm_instr(args: Pairs<Rule>) -> u32 {
        let args = Self::args_to_rule_str_pairs(args);

        let op = match args[0].0 {
            Rule::op_addi => 8,
            _ => panic!("Unexpected rule"),
        };

        let rt: u8 = args[1].1.parse().unwrap();
        let rs: u8 = args[2].1.parse().unwrap();
        let imm: i16 = args[3].1.parse().unwrap();

        eprintln!("PROCESSING IMMEDIATE {imm} aka {:#?}", args[3]);

        let instr = ITypeInstruction::new()
            .with_op(op)
            .with_rs(rs)
            .with_rt(rt)
            .with_imm(imm as u16);

        u32::from_le_bytes(instr.into_bytes())
    }

    fn encode_mem_instr(args: Pairs<Rule>) -> u32 {
        let args = Self::args_to_rule_str_pairs(args);

        let op = match args[0].0 {
            Rule::op_sw => 43,
            Rule::op_lw => 35,
            _ => panic!("Unexpected rule"),
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

    fn encode_arith_instr(args: Pairs<Rule>) -> u32 {
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

    fn encode_br_instr(&self, args: Pairs<Rule>) -> PotentialInstruction {
        let args = Self::args_to_rule_str_pairs(args);

        let op = match args[0].0 {
            Rule::op_beq => 4,
            _ => panic!("Unexpected rule"),
        };

        let rs: u8 = args[1].1.parse().unwrap();
        let rt: u8 = args[2].1.parse().unwrap();
        let label = &args[3].1;

        let instr = ITypeInstruction::new().with_op(op).with_rs(rs).with_rt(rt);
        PotentialInstruction::NeedsLabelResolution(
            TypedInstruction::IType(instr, self.current_addr),
            label.into(),
        )
    }

    fn encode_j_instr(args: Pairs<Rule>) -> PotentialInstruction {
        let args = Self::args_to_rule_str_pairs(args);

        let op = match args[0].0 {
            Rule::op_j => 2,
            _ => panic!("Unexpected rule"),
        };

        let label = &args[1].1;

        let instr = JTypeInstruction::new().with_op(op);
        PotentialInstruction::NeedsLabelResolution(TypedInstruction::JType(instr), label.into())
    }

    fn add_label(&mut self, label: &str) {
        if self.labels.contains_key(label) {
            panic!("Attempted to redefine label `{label}`");
        }

        self.labels.insert(label.into(), self.current_addr);
    }

    fn add_instruction(&mut self, instruction: Pair<Rule>) {
        let i = self.parse_instruction(instruction);

        self.instructions.push(i);
        self.current_addr += 4;
    }

    fn args_to_rule_str_pairs(args: Pairs<Rule>) -> Vec<(Rule, String)> {
        args.map(|p| (p.as_rule(), p.as_span().as_str().into()))
            .collect()
    }
}
