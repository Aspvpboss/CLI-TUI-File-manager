use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, ValueEnum, Ord, Debug)]
pub enum ArgFlags {
    USE_RECURSION,
}




#[derive(Parser)]
pub struct Args{

    #[command(subcommand)]
    command: Commands,

    /// flags used in CLI commands
    #[arg(short,long, value_enum)]
    pub flags: Vec<ArgFlags>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new file
    new {
        file_path: String,
    },
    /// Creates a new directory
    newf {
        dir_path: String,
    },
    /// Deletes a file
    del {
        file_path: String,
    },
    /// Deletes a directory, if the directory is not empty then it will fail unless the USE_RECURSION flag is used
    delf {
        dir_path: String,
    },
    /// Rename a file or directory, new name does not include path, just the new name
    rename {
        file_path: String,
        new_file_name: String,
    },
    /// Lists the contents of the provided directory 
    list {
        dir_path: String,
    },
    tui,
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