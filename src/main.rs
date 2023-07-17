use clap::{arg, command, value_parser, ArgAction, Command};

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
                .arg(
                    arg!(<package> "amount of full, final package.")
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    arg!(-d --daily "Use daily pay as basis for calculation.")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-f --fees "Take government fees for Iqama into account.")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches();

    match matches.subcommand().expect("required") {
        ("vat", args) => hsb::run_vat(args),
        ("salary", args) => hsb::run_salary(args),
        _ => unreachable!(),
    };
}
