use clap::Parser;
use::fm_cli::cli_args::{Args, Commands};
use std::process::ExitCode;

fn main() -> ExitCode {

    let args = Args::parse();

    match args.command {
        Commands::Tui => {
            if let Err(error) = fm_tui::run(){
                println!("{}", error);
                return ExitCode::FAILURE;            
            }
        }
        _ => {
            if let Err(error) = fm_cli::run(args){
                eprintln!("{}", error);
                return ExitCode::FAILURE;
            }
        }
    }

    return ExitCode::SUCCESS
}