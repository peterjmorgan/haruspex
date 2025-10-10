//! main.rs

use std::path::PathBuf;
use std::process;

use clap::Parser;

const PROGRAM: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = PROGRAM)]
#[command(version = VERSION)]
#[command(about = "Tool to extract IDA decompiler's pseudocode", long_about = None)]
struct Cli {
    #[arg(
        value_name = "FILE",
        help = "Binary file or IDA database (.i64/.idb) to analyze"
    )]
    input: PathBuf,

    #[arg(
        short = 'l',
        long = "list",
        help = "List all functions with their addresses and names"
    )]
    list: bool,

    #[arg(
        short = 'e',
        long = "load-existing",
        help = "Load existing IDA database if available (skips re-analysis)"
    )]
    load_existing: bool,

    #[arg(
        short = 'f',
        long = "function",
        value_name = "FUNC",
        help = "Decompile only the specified function (by name or address like 0x401000)"
    )]
    function: Option<String>,

    #[arg(
        short = 'o',
        long = "output",
        value_name = "PATH",
        help = "Output file path for single function decompilation"
    )]
    output: Option<PathBuf>,
}

fn main() {
    println!("{PROGRAM} {VERSION} - Tool to extract IDA decompiler's pseudocode");
    println!("Copyright (c) 2024-2025 Marco Ivaldi <raptor@0xdeadbeef.info>");
    println!();

    idalib::force_batch_mode();

    let cli = Cli::parse();

    let result = if cli.list {
        haruspex::list_functions(&cli.input, cli.load_existing)
    } else if let Some(func_spec) = cli.function {
        haruspex::run_single_function(
            &cli.input,
            cli.load_existing,
            &func_spec,
            cli.output.as_deref(),
        )
    } else {
        haruspex::run(&cli.input, cli.load_existing).map(|_| ())
    };

    match result {
        Ok(()) => (),
        Err(err) => {
            eprintln!("[!] Error: {err:#}");
            process::exit(1);
        }
    }
}
