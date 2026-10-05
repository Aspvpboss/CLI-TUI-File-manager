use clap::Parser;
use fm_cli::Args;


fn main() {

    let args = Args::parse();
    // don't panic
    let run_cli =  
        args.command.is_none() || 
        args.arg_one.is_none() || 
        args.arg_two.is_none(); 

    if run_cli == true {
        fm_cli::run();
    } else {
        fm_tui::run();
    }
}