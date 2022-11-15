use mipsasm::*;

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

#[test]
fn test_example_program() -> Result<(), MipsParserError> {
    let input = "
        main:   addi $2, $0, 5
                addi $3, $0, 12
                addi $7, $3, -9
                or   $4, $7, $2
                and  $5, $3, $4
                add  $5, $5, $4
                beq  $5, $7, end
                slt  $4, $3, $4
                beq  $4, $0, around
                addi $5, $0, 0
        around: slt  $4, $7, $2
                add  $7, $4, $5
                sub  $7, $7, $2
                sw   $7, 68($3)
                lw   $2, 80($0)
                j    end
                addi $2, $0, 1
        end:    sw   $2, 84($0)
        ";

    let expected: Vec<u32> = vec![
        0x20020005, 0x2003000c, 0x2067fff7, 0x00e22025, 0x00642824, 0x00a42820, 0x10a7000a,
        0x0064202a, 0x10800001, 0x20050000, 0x00e2202a, 0x00853820, 0x00e23822, 0xac670044,
        0x8c020050, 0x08000011, 0x20020001, 0xac020054,
    ];

    let instructions = MipsParser::parse_and_resolve_entire(input)?;

    assert!(do_vecs_match(&instructions, &expected));

    Ok(())
}

#[test]
fn test_instructions() -> Result<(), MipsParserError> {
    let cases = [
        ("add  $5, $5, $4", 0x00a42820),
        ("or   $4, $7, $2", 0x00e22025),
        ("and  $5, $3, $4", 0x00642824),
        ("slt  $4, $3, $4", 0x0064202a),
        ("sub  $7, $7, $2", 0x00e23822),
        ("addi $2, $0, 5", 0x20020005),
        ("addi $7, $3, -9", 0x2067fff7),
        ("sw   $7, 68($3)", 0xac670044),
        ("lw   $2, 80($0)", 0x8c020050),
    ];

    let assemble = |input: &str| -> Result<u32, MipsParserError> {
        let instrp = MipsParser::parse(Rule::instruction, input)?;
        let instrs: Vec<Pair<Rule>> = instrp.into_iter().collect();
        assert!(instrs.len() == 1);

        let i = instrs[0].clone(); // Rust won; I couldn't avoid the clone
        let p = MipsParser::new();

        match p.parse_instruction(i) {
            PotentialInstruction::Finished(hex) => Ok(hex),
            _ => panic!("Expected finished instruction"),
        }
    };

    for case in cases {
        assert!(assemble(case.0)? == case.1);
    }

    Ok(())
}
