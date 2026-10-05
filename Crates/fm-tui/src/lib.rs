use fm_core::FmError;


pub fn run() -> Option<FmError> {
    println!("fm-tui running on fm-core v{}", fm_core::version());

    None
}