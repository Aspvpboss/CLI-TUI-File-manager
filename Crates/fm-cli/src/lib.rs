use::clap::{Parser, ValueEnum};


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, ValueEnum, Ord, Debug)]
pub enum ArgFlags {
    BURGER,
}


#[derive(Parser)]
pub struct Args{
    /// args are new, newf, del, delf, rename, list 
    /// if none then it loads TUI
    pub command: Option<String>,

    /// argument used in the CLI commands
    pub arg_one: Option<String>,
    
    /// argument used in the CLI commands
    pub arg_two: Option<String>,

    /// flags used in CLI commands
    #[arg(short,long, value_enum)]
    pub flags: Vec<ArgFlags>,
}

pub fn run() {
    println!("fm-cli running on fm-core v{}", fm_core::version());
}