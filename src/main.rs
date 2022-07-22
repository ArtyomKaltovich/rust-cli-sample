use std::cmp::Ordering;
use taylor_series::exp;
use clap::Parser;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    /// argument of function to be called at
    #[clap(allow_hyphen_values = true)]
    x: f64,
    /// stop execution when difference between 2 step become lower then delta
    #[clap(validator = validate_delta)]
    delta: f64,
}

fn validate_delta(delta: &str) -> Result<(), String> {
    match delta.parse::<f64>().unwrap().total_cmp(&0.0) {
        Ordering::Greater => {Ok(())}
        _ => Err(String::from("delta should be positive"))
    }

}

fn main() {
    let args = Args::parse();
    println!("{}", exp(args.x, args.delta));
}
