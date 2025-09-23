mod header;

use anyhow::{Context, Error};
use clap::{Parser, Subcommand};
use header::Header;
use std::{fs::File, path::PathBuf, process::exit, str::FromStr};

#[derive(Parser)]
#[command(name = "neshdr", version)]
#[command(about = "Provide operations for NES 2.0 file header", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate NES 2.0 file header from definition file", long_about = None)]
    Gen {
        input: Option<PathBuf>,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    #[command(about = "Dump NES 2.0 file header into definition file", long_about = None)]
    Dump {
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    match parse_command(cli) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}

fn parse_command(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Commands::Gen { input, output } => {
            let input = input.unwrap_or(PathBuf::from_str("header.json")?);
            let output = output.unwrap_or(PathBuf::from_str("header.dat")?);
            do_gen(input, output)?;
        }
        Commands::Dump { input, output } => {
            let output = output.unwrap_or(PathBuf::from_str("header.json")?);
            do_dump(input, output)?;
        }
    }
    Ok(())
}

fn do_gen(input: PathBuf, output: PathBuf) -> Result<(), Error> {
    let input = File::open(&input)
        .with_context(|| format!("failed to open input file: {}", input.display()))?;
    let output = File::create(&output)
        .with_context(|| format!("failed to create output file: {}", output.display()))?;
    Header::from_json(input)?.into_bytes(output)?;
    Ok(())
}

fn do_dump(input: PathBuf, output: PathBuf) -> Result<(), Error> {
    let input = File::open(&input)
        .with_context(|| format!("failed to open input file: {}", input.display()))?;
    let output = File::create(&output)
        .with_context(|| format!("failed to create output file: {}", output.display()))?;
    Header::from_bytes(input)?.into_json(output)?;
    Ok(())
}
