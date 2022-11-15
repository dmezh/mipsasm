use mipsasm::*;

#[test]
fn test_instructions() {
    let cases = [
        ("add  $5, $5, $4", 0x00a42820),
        ("or   $4, $7, $2", 0x00e22025),
        ("and  $5, $3, $4", 0x00642824),
        ("slt  $4, $3, $4", 0x0064202a),
        ("sub  $7, $7, $2", 0x00e23822),
        ("addi $2, $0, 5",  0x20020005),
        ("addi $7, $3, -9", 0x2067fff7),
        ("sw   $7, 68($3)", 0xac670044),
        ("lw   $2, 80($0)", 0x8c020050),
    ];

    let assemble = |input: &str| {
        let instrp = MipsParser::parse(Rule::instruction, input).unwrap();
        let instrs: Vec<Pair<Rule>> = instrp.into_iter().collect();
        assert!(instrs.len() == 1);

        let i = instrs[0].clone(); // Rust won; I couldn't avoid the clone
        let p = MipsParser::new();

        match p.parse_instruction(i) {
            PotentialInstruction::Finished(hex) => hex,
            _ => panic!("Expected finished instruction"),
        }
    };

    for case in cases {
        assert!(assemble(case.0) == case.1);
    }
}
