use clap::{Parser, ValueEnum};
use std::fmt;

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


impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "command: {} - arg1: {} - arg2: {}", )

        let command = self.command.clone().unwrap_or("".to_string());
        let arg_one = self.arg_one.clone().unwrap_or("".to_string());
        let arg_two = self.arg_two.clone().unwrap_or("".to_string());


        write!(f, "command: {}, arg_one: {}, arg_two {}, flags {:?}", command, arg_one, arg_two, self.flags)
    }
}