use clap::Parser;

mod cli;
mod paths;
mod persistence;

fn main() {
    let cli = cli::Cli::parse();
    println!("{:?}", cli);
}
