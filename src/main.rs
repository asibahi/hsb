use clap::Parser;
use hsb::Commands::*;

fn main() {
    let cli = hsb::Cli::parse();
    match cli.command {
        Vat(args) => hsb::vat::run(args),
        Salary(args) => hsb::salary::run(args),
    }
}