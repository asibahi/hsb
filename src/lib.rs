use clap::ArgMatches;
use colored::Colorize;

const IQAMA_RENEWAL: f64 = 600.0;
const WORK_LICENSE: f64 = 9600.0;
const HEALTH_INSURANCE: f64 = 300.0;
const ANNUAL_FEES: f64 = IQAMA_RENEWAL + WORK_LICENSE + HEALTH_INSURANCE;

pub fn run_vat(args: &ArgMatches) {
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

pub fn run_salary(args: &ArgMatches) {
    let pack = *args.get_one::<usize>("package").unwrap();
    let d = args.get_flag("daily");
    let f = args.get_flag("fees");

    // making adjustments for the optional flags.
    let pack = if d { pack * 24 } else { pack } as f64;
    let pack = if f {
        pack - ((ANNUAL_FEES) / 12.0)
    } else {
        pack
    };

    // base math for base.
    // P = B (base salary) + B/4 (Housing allowance) + B/10 (Living allowance)
    // so B = 20 * P / 27
    let base = 20.0 * pack / 27.0;

    // round to nearest 100
    let base = (base / 100.0).round() * 100.0; // base salary
    let tprt = base / 10.0; // transport
    let hous = pack - (base + tprt); // housing

    println!("{:20}SAR {:.0}", "Full Package:", pack);
    if f {
        println!("{:20}SAR {:.0}", "Annual Fees:", ANNUAL_FEES);
    }
    println!(
        "{:20}SAR {}\n{:20}SAR {}\n{:20}SAR {}",
        "Base Salary:",
        format!("{:.0}", base).bold().green(),
        "Housing:",
        format!("{:.0}", hous).bold().green(),
        "Transport",
        format!("{:.0}", tprt).bold().green(),
    );
}
