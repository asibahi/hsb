use clap::Args;
use colored::Colorize;

#[derive(Args)]
pub struct VatArgs {
    /// amount of final bill
    amount: usize,

    /// VAT percentage
    #[arg(short = 't', long, default_value_t = 15)]
    vat: usize,
}

pub fn run(args: VatArgs) {
    let amount = args.amount;
    let vat = args.vat;

    let answer = amount as f64 / (1.0 + vat as f64 / 100.0);

    println!(
        "{:20}{}\n{:20}{}%\n{:20}{}",
        "Final Bill:",
        amount,
        "VAT:",
        vat,
        "Enter:",
        format!("{answer:.3}").bold().green()
    );
}