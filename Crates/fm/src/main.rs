use clap::{Parser};
use fm_cli::ArgFlags;

#[derive(Parser)]
struct Args{
    /// args are new, newf, del, delf, rename, list 
    /// if none then it loads TUI
    command: Option<String>,

    /// arguments used in the CLI commands
    arg: Option<String>,

    #[arg(short,long, value_enum)]
    flags: Vec<ArgFlags>,
}




fn main() {

    let args = Args::parse();
    let run_tui = false; 

    let command = match args.command {
        Some(command) => command,
        None => String::from(""),
    };
    let argument = match args.arg {
        Some(arg) => arg,
        None => String::from(""),
    };

    if std::env::args().nth(1).is_some() {
        fm_cli::run();
    } else {
        fm_tui::run();
    }
}