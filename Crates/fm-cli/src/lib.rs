use::clap::ValueEnum;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, ValueEnum, Ord, Debug)]
pub enum ArgFlags {
    BURGER,
}


pub fn run() {
    println!("fm-cli running on fm-core v{}", fm_core::version());
}