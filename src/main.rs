use std::fs;

use clap::Parser;
use neg_c::compile;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// compile a .nc file to a .piku file
    Build {
        /// the .nc file to compile
        file: String,
        /// print optional debug info
        #[arg(short, long)]
        debug: bool,
    },
    /// compile and run a .nc file
    Run {
        /// the .nc file to run
        file: String,
        /// print optional debug info
        #[arg(short, long)]
        debug: bool,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { file, debug } => {
            let _ = compile_file(file, *debug);
        },
        Commands::Run { file, debug } => {
            let compiled_file = match compile_file(file, *debug) {
                Ok(s) => s,
                Err(_) => return,
            };
            println!("{compiled_file}");

            let src = match fs::read_to_string(compiled_file) {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed to read file: {e}");
                    return;
                },
            };
            if let Err(e) = piku::run(src) {
                println!("The program encountered an error during runtime: {e}");
            }
        },
    }
}

/// Compile a .nc file into a .piku file.
/// Will return Err(()) if encountering an error and exiting early.
/// Will return Ok() with the name of compiled .piku file.
fn compile_file(file: &String, debug: bool) -> Result<String, ()> {
    let src = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to read file: {e}");
            return Err(());
        },
    };

    let asm = match compile(src, file.clone(), debug) {
        Some(s) => s,
        None => {
            println!("Error(s) occured, aborting compilation...");
            return Err(());
        },
    };

    let new_file = match get_file_name(file) {
        Some(s) => format!("{s}.piku"),
        None => {
            println!("Supplied file is not a .nc file");
            return Err(());
        }
    };
    if let Err(e) = fs::write(&new_file, asm) {
        println!("Failed to write file: {e}")
    }

    Ok(new_file)
}

fn get_file_name(file: &str) -> Option<&str> {
    let extension = &file[file.len() - 3..];
    if extension != ".nc" {
        return None;
    }
    Some(&file[..file.len() - 3])
}
