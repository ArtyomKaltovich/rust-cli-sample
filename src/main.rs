use taylor_series::exp;
use clap::Parser;


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    /// argument of function to be called at
    #[clap(allow_hyphen_values = true)]
    x: f64,
    /// stop execution when difference between 2 step become lower then delta
    delta: f64,
}

fn main() {
    let args = Args::parse();
    println!("{}", exp(args.x, args.delta));
}
