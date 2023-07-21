use clap::Args;
use colored::Colorize;

const IQAMA_RENEWAL: f64 = 600.0;
const WORK_LICENSE: f64 = 9600.0;
const HEALTH_INSURANCE: f64 = 300.0;
const ANNUAL_FEES: f64 = IQAMA_RENEWAL + WORK_LICENSE + HEALTH_INSURANCE;
const MONTHLY_FEES: f64 = ANNUAL_FEES / 12.0;

#[derive(Args)]
pub struct SalaryArgs {
    /// amount of fullm final package.
    package: usize,
    /// Use daily pay as basis for calculation.
    #[arg(short, long)]
    daily: bool,

    /// Take government fees for Iqama into account.
    #[arg(short, long)]
    fees: bool,
}

pub fn run(args: SalaryArgs) {
    let pack = args.package;
    
    // making adjustments for the optional flags.
    let pack = if args.daily { pack * 24 } else { pack } as f64;
    let pack = if args.fees { pack - MONTHLY_FEES } else { pack };

    // base math for base.
    // P = B (base salary) + B/4 (Housing allowance) + B/10 (Living allowance)
    // so B = 20 * P / 27
    let base = 20.0 * pack / 27.0;

    // round to nearest 100
    let base = (base / 100.0).round() * 100.0; // base salary
    let tprt = base / 10.0; // transport
    let hous = pack - (base + tprt); // housing

    println!("{:20}SAR {:.0}", "Full Package:", pack);
    if args.fees {
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
