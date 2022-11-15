use anyhow::{Context, Result};
use clap::Parser as ClapParser;

use mipsasm::*;

#[derive(clap::Parser)]
struct Args {
    /// Input filename
    in_filename: String,

    /// Output filename
    out_filename: String,

    /// Append '0x' prefix to each instruction
    #[arg(short, long)]
    prefix: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.in_filename)
        .context(format!("Could not read input file `{}`", args.in_filename))?;

    let instructions = MipsParser::parse_and_resolve_entire(&input)
        .context(format!("Could not parse input file"))?;

    let mut output = String::new();

    let usr_format = if args.prefix {
        |h| format!("0x{:08x}\n", h)
    } else {
        |h| format!("{:08x}\n", h)
    };

    instructions
        .iter()
        .for_each(|h| output.push_str(&usr_format(h)));

    std::fs::write(&args.out_filename, output).context(format!(
        "Could not write output file `{}`",
        args.out_filename
    ))?;

    Ok(())
}
