
fn main() {
    if std::env::args().nth(1).is_some() {
        fm_cli::run();
    } else {
        fm_tui::run();
    }

}