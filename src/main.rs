mod machine;

use anyhow::Ok;
use clap::Parser;
use machine::Machine;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// program file
    #[arg(value_name = "FILE")]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let bin = std::fs::read(cli.path)?;
    let mut machine = Machine::new(2 << 20);
    machine.load_bin(0, &bin)?;
    machine.run()?;
    machine.debug();
    Ok(())
}
