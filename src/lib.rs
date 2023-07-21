use clap::{Parser, Subcommand};

pub mod salary;
pub mod vat;

#[derive(Parser)]
#[command(author, version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get value without VAT from a final value with VAT
    Vat(vat::VatArgs),
    /// Get salary devisions from a final salary
    Salary(salary::SalaryArgs),
}
