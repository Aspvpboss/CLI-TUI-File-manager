
pub mod cli_args;

use cli_args::{Args, Commands};
use fm_core::*;

pub fn run(args: Args) -> Result<()> {

    match args.command {
        Commands::New { file_path } => {
            fm_core::create_file(file_path)?;
        }
        Commands::Newd { dir_path } => {
            fm_core::create_dir(dir_path)?;
        }
        Commands::Del { file_path } => {
            fm_core::remove_file(file_path)?;
        }
        Commands::Deld { dir_path, recursion } => {
            let recursion = if recursion {
                Recursion::Yes
            } else {
                Recursion::No
            };

            fm_core::remove_dir(dir_path, recursion)?;
        }
        Commands::Rename { file_path, new_file_name } => {
            fm_core::rename(file_path, new_file_name)?;
        }
        Commands::List { dir_path } => {
            let dir_list = fm_core::list_dir(dir_path)?;
            for entry in dir_list {
                println!("{}", entry);
            }
        }
        Commands::Copy { path } => {
            fm_core::copy_files(vec![path])?;
        }
        Commands::Paste { dir_path } => {
            fm_core::paste_files(dir_path)?;
        }
        _ => {
            return Err(FmError::Cli("Invalid command".to_string()));
        }
    }

    Ok(())
}