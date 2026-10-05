pub mod cli_args;


pub fn run() {
    println!("fm-cli running on fm-core v{}", fm_core::version());
}