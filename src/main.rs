use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command you want the file manager to do
    // #[arg(short, long)]
    command: Option<String>,

    /// Arg for the command to use
    // #[arg(short, long)]
    arg: Option<String>,
   
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

#[derive(Debug)]
enum ArgState {
    CLI,
    TUI
}

struct ProcessedArgs {
    command: String,
    arg: String,
    state: ArgState,
}

fn process_args() -> ProcessedArgs {

    let args = Args::parse();

    let mut processed_args = ProcessedArgs {
        command : String::from(""),
        arg : String::from(""),
        state : ArgState::TUI,
    };

    match args.command {
        Some(command) => {
            processed_args.command = command;
            processed_args.state = ArgState::CLI;
        },
        None => {}
    }
    match args.arg {
        Some(arg) => {
            processed_args.arg = arg;
            processed_args.state = ArgState::CLI;
        },
        None => {}
    }

    return processed_args
}


fn main() {
    println!("fm-cli running on fm-core v{}", fm_core::version());

    let args = process_args();
    
    println!("{}, {}, {:?}", args.command, args.arg, args.state);
}