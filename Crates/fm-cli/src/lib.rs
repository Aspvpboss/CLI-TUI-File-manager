pub mod cli_args;
use cli_args::{Args, ArgFlags, Commands};
use fm_core::*;

pub fn run(args: Args) -> Result<()> {
    // Match directly on the clap Subcommand enum
    match args.command {
        Commands::new { file_path } => {
            fm_core::create_file(file_path)?;
        }
        Commands::newf { dir_path } => {
            fm_core::create_dir(dir_path)?;
        }
        Commands::del { file_path } => {
            fm_core::remove_file(file_path)?;
        }
        Commands::delf { dir_path, recursion } => {
            let recursion = if recursion {
                Recursion::Yes
            } else {
                Recursion::No
            };

            fm_core::remove_dir(dir_path, recursion)?;
        }
        Commands::rename { file_path, new_file_name } => {
            fm_core::rename(file_path, new_file_name)?;
        }
        Commands::list { dir_path } => {
            let dir_list = fm_core::list_dir(dir_path)?;
            for entry in dir_list {
                println!("{}", entry);
            }
        }
        Commands::tui => {
            // Placeholder for TUI execution
            println!("TUI interface launching...");
        }
    }

    Ok(())
}