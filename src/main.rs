use forger::Forger;

pub mod argparser;
pub mod commander;
pub mod filehandler;
pub mod forger;
pub mod interpreter;
pub mod logging;
pub mod parser;
pub mod variables;
pub mod constants;
pub mod help;

fn main() {
    let mut forger: Forger = Forger::new();
    forger.run();
}
