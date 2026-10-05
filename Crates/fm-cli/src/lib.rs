pub mod cli_args;
pub mod cli_config;
use cli_args::Args;
use cli_config::CommandExecuteConfigs;
use fm_core::*;

pub fn run(args : Args) -> Result<()>{


    let command = args.command.unwrap_or("".to_string());
    let arg_one = args.arg_one.unwrap_or("".to_string());
    let arg_two = args.arg_two.unwrap_or("".to_string());


    // we can add some way to change the hardcoded strings in this later 
    let processed_command = match command.as_str() {
        "new" => Some(CommandExecuteConfigs::NEW),
        "newf" => Some(CommandExecuteConfigs::NEWF), // Assuming these variants exist
        "del" => Some(CommandExecuteConfigs::DEL),
        "delf" => Some(CommandExecuteConfigs::DELF),
        "rename" => Some(CommandExecuteConfigs::RENAME),
        "list" => Some(CommandExecuteConfigs::LIST),
        _ => None,
    };

    // Now handle execution based on the config/enum
    match processed_command {
        Some(CommandExecuteConfigs::NEW) => {
            fm_core::create_file(arg_one)?;
        }
        Some(CommandExecuteConfigs::NEWF) => {
            fm_core::create_dir(arg_one)?;
        }
        Some(CommandExecuteConfigs::DEL) => {
            // Handle del
        }
        Some(CommandExecuteConfigs::DELF) => {
            // Handle delf
        }
        Some(CommandExecuteConfigs::RENAME) => {
            // Handle rename
        }
        Some(CommandExecuteConfigs::LIST) => {
            // Handle list
        }
        None => {
            return Err(FmError::Cli("invalid command arg given".to_string()));
        }
    }

    Ok(())
}