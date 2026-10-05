use clap::Parser;
use::fm_cli::cli_args::Args;


fn main() {

    let args = Args::parse();
    // don't panic
    let run_tui =  
        args.command.is_none() && 
        args.arg_one.is_none() && 
        args.arg_two.is_none(); 

    if run_tui == true {
        fm_tui::run();
    } else {
        fm_cli::run();
    }
}