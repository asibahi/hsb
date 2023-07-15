use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use colored::Colorize;

fn main() {
    let matches = command!()
        .author("Abdul Rahman Sibahi")
        .subcommand_required(true)
        // VAT Command
        .subcommand(
            Command::new("vat")
                .about("Get value without VAT from a final value with VAT")
                .arg(arg!(<amount> "amount of final bill").value_parser(value_parser!(usize)))
                .arg(
                    arg!(-t --vat <vat> "VAT percentage")
                        .value_parser(value_parser!(usize))
                        .default_value("15"),
                ),
        )
        // Salary Command
        .subcommand(
            Command::new("salary")
                .about("Get salary devisions from a final salary")
                .arg(arg!(<package> "amount of full package").value_parser(value_parser!(usize)))
                .arg(arg!(-d --daily "Use daily pay as basis").action(ArgAction::SetTrue)),
        )
        .get_matches();

    match matches.subcommand().expect("required") {
        ("vat", args) => run_vat(args),
        ("salary", args) => run_salary(args),
        _ => unreachable!(),
    };
}

fn run_salary(args: &ArgMatches) {
    let p = *args.get_one::<usize>("package").unwrap();
    let p = if args.get_flag("daily") { p * 24 } else { p } as f64;

    // base math for base.
    // P = B (base salary) + B/4 (Housing allowance) + B/10 (Living allowance)
    // so B = 20 * P / 27
    let b = 20.0 * p / 27.0;

    // round to nearest 100
    let b = (b / 100.0).round() * 100.0; // base salary
    let t = b / 10.0; // transport
    let h = p - (b + t); // housing

    println!(
        "{:20}SAR {:.0}\n{:20}SAR {}\n{:20}SAR {}\n{:20}SAR {}",
        "Full Package:",
        p,
        "Base Salary:",
        format!("{:.0}", b).bold().green(),
        "Housing:",
        format!("{:.0}", h).bold().green(),
        "Transport",
        format!("{:.0}", t).bold().green(),
    );
}

fn run_vat(args: &ArgMatches) {
    let amount = *args.get_one::<usize>("amount").unwrap();
    let vat = *args.get_one::<usize>("vat").unwrap();

    let answer = amount as f64 / (1.0 + vat as f64 / 100.0);

    println!(
        "{:20}{}\n{:20}{}%\n{:20}{}",
        "Final Bill:",
        amount,
        "VAT:",
        vat,
        "Enter:",
        format!("{:.3}", answer).bold().green()
    );
}
