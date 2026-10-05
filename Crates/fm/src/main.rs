use clap::Parser;

#[derive(Parser)]
struct Args{
    /// args are new, newf, del, delf, rename, list 
    /// if none then it loads tui
    command: Option<String>,

    arg: Option<String>,
}




fn main() {
    if std::env::args().nth(1).is_some() {
        fm_cli::run();
    } else {
        fm_tui::run();
    }
}