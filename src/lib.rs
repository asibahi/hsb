use anyhow::Result;
use clap::{Parser, Subcommand};

mod mimiron;
mod salary;
mod vat;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get value without VAT from a final value with VAT
    Vat(vat::VatArgs),

    /// Get salary devisions from a final salary
    Salary(salary::SalaryArgs),

    /// Make Hearthstone API enquiries. Work in progress.
    Mimiron(mimiron::MimironArgs),
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Vat(args) => vat::run(args),
        Commands::Salary(args) => salary::run(args),
        Commands::Mimiron(args) => mimiron::run(args)?,
    };

    Ok(())
}
