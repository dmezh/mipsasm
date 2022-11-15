use mipsasm::*;

#[test]
fn test_arith_ops() {
    let cases = [
        ("add $5, $5, $4", 0x00a42820),
        ("or  $4, $7, $2", 0x00e22025),
    ];

    let assemble = |input: &str| {
        let instrp = MipsParser::parse(Rule::instruction, input).unwrap();
        let instrs: Vec<Pair<Rule>> = instrp.into_iter().collect();
        assert!(instrs.len() == 1);

        let i = instrs[0].clone(); // Rust won; I couldn't avoid the clone
        MipsParser::parse_instruction(i)
    };

    for case in cases {
        assert!(assemble(case.0) == case.1);
    }
}
