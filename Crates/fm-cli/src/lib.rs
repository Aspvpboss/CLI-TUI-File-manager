use clap::Parser;

#[derive(Parser)]
struct Args{
    /// args are new, newf, del, delf, rename, list 
    /// if none then it loads tui
    command: Option<String>,

    arg: Option<String>,
}





pub fn run() {
    println!("fm-cli running on fm-core v{}", fm_core::version());
}