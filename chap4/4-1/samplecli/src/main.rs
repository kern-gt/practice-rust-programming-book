use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "sh-goto",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[arg(short, long)]
    verbose: bool,

    /// Formulas written in RPN
    #[arg(name = "FILE")]
    formula_file: Option<String>,
}

fn main () {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("Is verbosity specified?: {}", opts.verbose);
}