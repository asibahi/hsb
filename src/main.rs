use clap::Parser;
use hsb::*;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Vat(args) => vat::run(args),
        Commands::Salary(args) => salary::run(args),
    }
}
