use clap::Parser;
use::fm_cli::cli_args::Args;
use std::process::ExitCode;

fn main() -> ExitCode {

    let args = Args::parse();
    // don't panic
    let run_tui =  
        args.command.is_none() && 
        args.arg_one.is_none() && 
        args.arg_two.is_none(); 

    if run_tui == true {
        if let Some(error) = fm_tui::run(){
            println!("{}", error);
            return ExitCode::FAILURE;            
        }

        return ExitCode::FAILURE;            
    } else {
        if let Some(error) = fm_cli::run(args){
            println!("{}", error);
            return ExitCode::FAILURE;
        }
    }
    return ExitCode::SUCCESS
}