pub mod cli_args;
use cli_args::Args;


pub fn run(args : Args) {
    println!("fm-cli running on fm-core v{}", fm_core::version());
    println!("{}", args);
}